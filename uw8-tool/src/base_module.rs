use std::{collections::HashMap, fs::File, path::Path};

use anyhow::{bail, Result};
use std::io::prelude::*;
use wasm_encoder::{
    CodeSection, EntityType, Export, ExportSection, Function, FunctionSection, ImportSection,
    Instruction, MemoryType, Module, TypeSection, ValType,
};
use ValType::*;

pub struct BaseModule {
    pub types: Vec<FunctionType>,
    pub function_imports: Vec<(&'static str, String, u32)>,
    pub global_imports: Vec<(&'static str, String, GlobalType)>,
    pub functions: Vec<u32>,
    pub exports: Vec<(&'static str, u32)>,
    pub memory: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalType {
    pub type_: ValType,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub params: Vec<ValType>,
    pub result: Option<ValType>,
}

impl BaseModule {
    pub fn for_format_version(version: u8) -> Result<BaseModule> {
        if version != 1 {
            bail!("Unsupported format version ({})", version);
        }

        let mut types = vec![];
        let mut type_map = HashMap::new();
        for num_params in 0..6 {
            for num_f32 in 0..=num_params {
                for &result in &[None, Some(ValType::I32), Some(ValType::F32)] {
                    let mut params = vec![];
                    for _ in 0..num_f32 {
                        params.push(F32);
                    }
                    for _ in num_f32..num_params {
                        params.push(I32);
                    }
                    let type_ = FunctionType { params, result };
                    type_map.insert(type_.clone(), types.len() as u32);
                    types.push(type_);
                }
            }
        }

        let mut functions = vec![];
        add_function(&mut functions, &type_map, "sin", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "cos", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "tan", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "asin", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "acos", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "atan", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "atan2", &[F32, F32], Some(F32));
        add_function(&mut functions, &type_map, "pow", &[F32, F32], Some(F32));
        add_function(&mut functions, &type_map, "log", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "fmod", &[F32, F32], Some(F32));

        add_function(&mut functions, &type_map, "random", &[], Some(I32));
        add_function(&mut functions, &type_map, "randomf", &[], Some(F32));
        add_function(&mut functions, &type_map, "randomSeed", &[I32], None);

        add_function(&mut functions, &type_map, "cls", &[I32], None);
        add_function(&mut functions, &type_map, "setPixel", &[I32, I32, I32], None);
        add_function(&mut functions, &type_map, "getPixel", &[I32, I32], Some(I32));
        add_function(&mut functions, &type_map, "hline", &[I32, I32, I32, I32], None);
        add_function(&mut functions, &type_map, "rectangle", &[F32, F32, F32, F32, I32], None);
        add_function(&mut functions, &type_map, "circle", &[F32, F32, F32, I32], None);

        for i in functions.len()..64 {
            add_function(
                &mut functions,
                &type_map,
                &format!("reserved{}", i),
                &[],
                None,
            );
        }

        let mut global_imports = vec![];
        for i in 0..16 {
            global_imports.push((
                "env",
                format!("g_reserved{}", i),
                GlobalType {
                    type_: I32,
                    mutable: false,
                },
            ));
        }

        let first_function = functions.len() as u32;

        Ok(BaseModule {
            types,
            function_imports: functions,
            global_imports,
            functions: vec![lookup_type(&type_map, &[I32], None)],
            exports: vec![("tic", first_function)],
            memory: 4,
        })
    }

    pub fn to_wasm(&self) -> Vec<u8> {
        let mut module = Module::new();

        {
            let mut types = TypeSection::new();
            for type_ in &self.types {
                types.function(type_.params.iter().cloned(), type_.result.iter().cloned());
            }
            module.section(&types);
        }

        {
            let mut imports = ImportSection::new();

            for (module, name, type_) in &self.function_imports {
                imports.import(*module, Some(name.as_str()), EntityType::Function(*type_));
            }

            for (module, name, import) in &self.global_imports {
                imports.import(
                    *module,
                    Some(name.as_str()),
                    EntityType::Global(wasm_encoder::GlobalType {
                        val_type: import.type_,
                        mutable: import.mutable,
                    }),
                );
            }

            imports.import(
                "env",
                Some("memory"),
                MemoryType {
                    minimum: self.memory as u64,
                    maximum: None,
                    memory64: false,
                },
            );

            module.section(&imports);
        }

        {
            let mut functions = FunctionSection::new();

            for type_ in &self.functions {
                functions.function(*type_);
            }

            module.section(&functions);
        }

        {
            let mut exports = ExportSection::new();

            for (name, fnc) in &self.exports {
                exports.export(*name, Export::Function(*fnc));
            }

            module.section(&exports);
        }

        {
            let mut code = CodeSection::new();

            for _ in &self.functions {
                let mut function = Function::new([]);
                function.instruction(&Instruction::End);
                code.function(&function);
            }

            module.section(&code);
        }

        module.finish()
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        File::create(path)?.write_all(&self.to_wasm())?;
        Ok(())
    }
}

fn add_function(
    functions: &mut Vec<(&'static str, String, u32)>,
    type_map: &HashMap<FunctionType, u32>,
    name: &str,
    params: &[ValType],
    result: Option<ValType>,
) {
    functions.push((
        "env".into(),
        name.to_string(),
        lookup_type(type_map, params, result),
    ));
}

fn lookup_type(
    type_map: &HashMap<FunctionType, u32>,
    params: &[ValType],
    result: Option<ValType>,
) -> u32 {
    let key = FunctionType {
        params: params.to_vec(),
        result,
    };
    *type_map.get(&key).unwrap()
}
