use std::{collections::HashMap, fs::File, path::Path};

use wasm_encoder::{
    CodeSection, EntityType, Export, ExportSection, Function, FunctionSection,
    ImportSection, Instruction, MemoryType, Module, TypeSection, ValType,
};
use ValType::*;
use anyhow::{Result, bail};
use std::io::prelude::*;

pub struct BaseModule {
    pub types: Vec<FunctionType>,
    pub function_imports: Vec<(&'static str, &'static str, u32)>,
    pub functions: Vec<u32>,
    pub exports: Vec<(&'static str, u32)>,
    pub memory: u32
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub params: Vec<ValType>,
    pub result: Option<ValType>
}

impl BaseModule {
    pub fn for_format_version(version: u32) -> Result<BaseModule> {
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
        add_function(&mut functions, &type_map, "math","sin", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "cos", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "tan", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "asin", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "acos", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "atan", &[F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "atan2", &[F32, F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "pow", &[F32, F32], Some(F32));
        add_function(&mut functions, &type_map, "math", "log", &[F32], Some(F32));

        let first_function = functions.len() as u32;

        Ok(BaseModule {
            types,
            function_imports: functions,
            functions: vec![lookup_type(&type_map, &[I32], None)],
            exports: vec![("tic", first_function)],
            memory: 4
        })
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fn inner(m: &BaseModule, path: &Path) -> Result<()> {
            let mut module = Module::new();

            {
                let mut types = TypeSection::new();
                for type_ in &m.types {
                    types.function(type_.params.iter().cloned(), type_.result.iter().cloned());
                }
                module.section(&types);
            }

            {
                let mut imports = ImportSection::new();

                for (module, name, type_) in &m.function_imports {
                    imports.import(*module, Some(*name), EntityType::Function(*type_));
                }

                imports.import("env", Some("memory"), MemoryType {
                    minimum: m.memory as u64,
                    maximum: None,
                    memory64: false
                });

                module.section(&imports);
            }

            {
                let mut functions = FunctionSection::new();

                for type_ in &m.functions {
                    functions.function(*type_);
                }

                module.section(&functions);
            }

            {
                let mut exports = ExportSection::new();

                for (name, fnc) in &m.exports {
                    exports.export(*name, Export::Function(*fnc));
                }

                module.section(&exports);
            }

            {
                let mut code = CodeSection::new();

                for _ in &m.functions {
                    let mut function = Function::new([]);
                    function.instruction(&Instruction::End);
                    code.function(&function);
                }

                module.section(&code);
            }

            let data = module.finish();

            File::create(path)?.write_all(&data)?;

            Ok(())
        }
        inner(self, path.as_ref())
    }
}

fn add_function(functions: &mut Vec<(&'static str, &'static str, u32)>, type_map: &HashMap<FunctionType, u32>, module: &'static str, name: &'static str, params: &[ValType], result: Option<ValType>) {
    functions.push((module, name, lookup_type(type_map, params, result)));
}

fn lookup_type(type_map: &HashMap<FunctionType, u32>, params: &[ValType], result: Option<ValType>) -> u32 {
    let key = FunctionType {
        params: params.to_vec(),
        result
    };
    *type_map.get(&key).unwrap()
}