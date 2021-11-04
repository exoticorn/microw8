use crate::base_module::{self, BaseModule, FunctionType};
use anyhow::{anyhow, bail, Result};
use enc::ValType;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};
use wasm_encoder as enc;
use wasmparser::{
    ExportSectionReader, ExternalKind, FunctionBody, FunctionSectionReader, ImportSectionEntryType,
    ImportSectionReader, TypeSectionReader,
};

pub fn pack(source: &Path, dest: &Path, version: u32) -> Result<()> {
    let base = BaseModule::for_format_version(version)?;

    let mut source_data = vec![];
    File::open(source)?.read_to_end(&mut source_data)?;

    let parsed_module = dbg!(ParsedModule::parse(&source_data)?);
    let result = parsed_module.pack(&base)?;

    /*
    let module = enc::Module::new();

    let mut context = Context {
        base,
        module,
        type_mapping: HashMap::new(),
        function_mapping: HashMap::new(),
        next_function_index: 0,
    };

    for payload in wasmparser::Parser::new(0).parse_all(&source_data) {
        let payload = payload?;

        use wasmparser::Payload::*;

        let _copy = match payload {
            Version { .. } => false,
            TypeSection(reader) => process_type_section(&mut context, reader)?,
            ImportSection(reader) => process_import_section(&mut context, reader)?,
            _ => true,
        };
    }
    */

    let mut dest_data = vec![version as u8];
    dest_data.extend_from_slice(&result[8..]);
    File::create(dest)?.write_all(&dest_data)?;

    Ok(())
}

struct Context {
    base: BaseModule,
    module: enc::Module,
    type_mapping: HashMap<u32, u32>,
    function_mapping: HashMap<u32, u32>,
    next_function_index: u32,
}

fn process_type_section(
    context: &mut Context,
    mut reader: wasmparser::TypeSectionReader,
) -> Result<bool> {
    use wasmparser::TypeDef;

    let base_function_types: HashMap<base_module::FunctionType, u32> = context
        .base
        .types
        .iter()
        .enumerate()
        .map(|(i, t)| (t.clone(), i as u32))
        .collect();

    for src_index in 0..reader.get_count() {
        match reader.read()? {
            TypeDef::Func(fnc) => {
                let params: Vec<ValType> = to_val_type_vec(&fnc.params)?;
                if fnc.returns.len() > 1 {
                    bail!("Multi-value not yet supported, sorry.");
                }
                let result = to_val_type_vec(&fnc.returns)?.into_iter().next();

                let func_type = base_module::FunctionType { params, result };
                if let Some(index) = base_function_types.get(&func_type) {
                    context.type_mapping.insert(src_index, *index);
                } else {
                    bail!("Function type not found in base: {:?}", func_type);
                }
            }
            TypeDef::Instance(_) | TypeDef::Module(_) => {
                bail!("Instance and module types are not supported")
            }
        }
    }

    Ok(false)
}

fn process_import_section(
    context: &mut Context,
    mut reader: wasmparser::ImportSectionReader,
) -> Result<bool> {
    let mut found_memory = false;

    let function_import_map: HashMap<(String, String), (u32, u32)> = context
        .base
        .function_imports
        .iter()
        .enumerate()
        .map(|(index, (module, name, type_))| {
            ((module.to_string(), name.clone()), (*type_, index as u32))
        })
        .collect();

    for _ in 0..reader.get_count() {
        let import = reader.read()?;
        if let Some(field) = import.field {
            match import.ty {
                ImportSectionEntryType::Memory(ref mem) => {
                    if import.module != "env" || field != "memory" {
                        bail!("Wrong import name for memory: {}.{}", import.module, field);
                    }
                    if mem.memory64 || mem.shared {
                        bail!("Bad memory options");
                    }
                    if mem.initial > context.base.memory as u64
                        || mem.maximum.unwrap_or(0) > context.base.memory as u64
                    {
                        bail!("Import memory too large");
                    }
                    found_memory = true;
                }
                ImportSectionEntryType::Function(type_) => {
                    if let Some((base_type, base_index)) =
                        function_import_map.get(&(import.module.to_string(), field.to_string()))
                    {
                        if let Some(type_) = context.type_mapping.get(&type_) {
                            if type_ == base_type {
                                context
                                    .function_mapping
                                    .insert(context.next_function_index, *base_index);
                                context.next_function_index += 1;
                            } else {
                                bail!("Import function {}.{} has wrong type", import.module, field);
                            }
                        } else {
                            bail!("Malformed input (Type {} does not exist)", type_);
                        }
                    } else {
                        bail!(
                            "Import function {}.{} does not exist in base",
                            import.module,
                            field
                        );
                    }
                }
                _ => bail!("Unsupported import type {:?}", import.ty),
            }
        } else {
            bail!("Found import without field: {}", import.module);
        }
    }

    if !found_memory {
        bail!("No memory import found");
    }

    Ok(false)
}

fn to_val_type(type_: &wasmparser::Type) -> Result<ValType> {
    use wasmparser::Type::*;
    Ok(match *type_ {
        I32 => ValType::I32,
        I64 => ValType::I64,
        F32 => ValType::F32,
        F64 => ValType::F64,
        _ => bail!("Type {:?} isn't a value type", type_),
    })
}

fn to_val_type_vec(types: &[wasmparser::Type]) -> Result<Vec<ValType>> {
    types.into_iter().map(to_val_type).collect()
}

#[derive(Debug)]
struct ParsedModule<'a> {
    data: &'a [u8],
    types: Section<Vec<base_module::FunctionType>>,
    imports: Section<ImportSection>,
    functions: Section<Vec<u32>>,
    exports: Section<Vec<(String, u32)>>,
    function_bodies: Vec<wasmparser::FunctionBody<'a>>,
}

impl<'a> ParsedModule<'a> {
    fn parse(data: &'a [u8]) -> Result<ParsedModule<'a>> {
        let mut parser = wasmparser::Parser::new(0);

        let mut type_section = None;
        let mut import_section = None;
        let mut function_section = None;
        let mut export_section = None;
        let mut function_bodies = Vec::new();

        let mut offset = 0;

        loop {
            let (consumed, payload) = if let wasmparser::Chunk::Parsed { consumed, payload } =
                parser.parse(&data[offset..], true)?
            {
                (consumed, payload)
            } else {
                unreachable!();
            };

            use wasmparser::Payload;

            let range = offset..(offset + consumed);

            match payload {
                Payload::Version { .. } => (),
                Payload::TypeSection(reader) => {
                    type_section = Some(Section::new(range, read_type_section(reader)?));
                }
                Payload::ImportSection(reader) => {
                    import_section = Some(Section::new(range, ImportSection::parse(reader)?));
                }
                Payload::FunctionSection(reader) => {
                    function_section = Some(Section::new(range, read_function_section(reader)?));
                }
                Payload::ExportSection(reader) => {
                    export_section = Some(Section::new(range, read_export_section(reader)?));
                }
                Payload::CodeSectionStart { .. } => (),
                Payload::CodeSectionEntry(body) => function_bodies.push(body),
                Payload::CustomSection { .. } => (),
                Payload::End => break,
                other => bail!("Unsupported section: {:?}", other),
            }

            dbg!(consumed);

            offset += consumed;
        }

        Ok(ParsedModule {
            data,
            types: type_section.ok_or_else(|| anyhow!("No type section found"))?,
            imports: import_section.ok_or_else(|| anyhow!("No import section found"))?,
            functions: function_section.ok_or_else(|| anyhow!("No function section found"))?,
            exports: export_section.ok_or_else(|| anyhow!("No export section found"))?,
            function_bodies,
        })
    }

    fn pack(self, base: &BaseModule) -> Result<Vec<u8>> {
        let mut module = enc::Module::new();

        let mut type_map = HashMap::new();

        let mut uses_base_types = true;

        {
            let base_type_map: HashMap<FunctionType, u32> = base
                .types
                .iter()
                .enumerate()
                .map(|(idx, type_)| (type_.clone(), idx as u32))
                .collect();

            for (idx, type_) in self.types.data.iter().enumerate() {
                if let Some(base_idx) = base_type_map.get(type_) {
                    type_map.insert(idx as u32, *base_idx);
                } else {
                    println!("Type {:?} not found in base", type_);
                    uses_base_types = false;
                }
            }

            if !uses_base_types {
                type_map = (0..self.types.data.len() as u32).map(|i| (i, i)).collect();

                copy_section(&mut module, &self.data[self.types.range.clone()])?;
            }
        }

        let mut function_map = HashMap::new();
        let mut function_count = 0;

        if uses_base_types {
            if self.imports.data.memory > base.memory {
                bail!(
                    "Trying to import more memory than base ({})",
                    self.imports.data.memory
                );
            }

            let base_import_map: HashMap<(String, String), (u32, u32)> = base
                .function_imports
                .iter()
                .enumerate()
                .map(|(idx, (module, field, type_))| {
                    ((module.to_string(), field.clone()), (*type_, idx as u32))
                })
                .collect();

            for (idx, fnc) in self.imports.data.functions.iter().enumerate() {
                if let Some((base_type, base_idx)) =
                    base_import_map.get(&(fnc.module.clone(), fnc.field.clone()))
                {
                    if type_map.get(&fnc.type_) == Some(base_type) {
                        function_map.insert(idx as u32, *base_idx);
                    } else {
                        bail!(
                            "Import function {}.{} has incompatible type",
                            fnc.module,
                            fnc.field
                        );
                    }
                } else {
                    bail!(
                        "Import function {}.{} not found in base",
                        fnc.module,
                        fnc.field
                    );
                }
            }

            function_count += base.function_imports.len();
        } else {
            function_map = (0..self.imports.data.functions.len() as u32)
                .map(|i| (i, i))
                .collect();

            copy_section(&mut module, &self.data[self.imports.range.clone()])?;

            function_count += self.imports.data.functions.len();
        }

        // TODO: re-order functions to put tic first
        for i in 0..self.functions.data.len() as u32 {
            function_map.insert(
                self.imports.data.functions.len() as u32 + i,
                function_count as u32,
            );
            function_count += 1;
        }

        if self.functions.data.len() != base.functions.len()
            || self
                .functions
                .data
                .iter()
                .zip(base.functions.iter())
                .any(|(a, b)| type_map.get(a) != Some(b))
        {
            let mut function_section = enc::FunctionSection::new();
            for type_ in &self.functions.data {
                function_section.function(
                    *type_map
                        .get(type_)
                        .ok_or_else(|| anyhow!("Type index out of range: {}", type_))?,
                );
            }
            module.section(&function_section);
        }

        {
            let mut base_exports = base.exports.clone();
            base_exports.sort();

            let mut my_exports: Vec<(String, u32)> = self
                .exports
                .data
                .iter()
                .map(|(name, func)| (name.clone(), *function_map.get(func).unwrap()))
                .collect();
            my_exports.sort();

            if base_exports
                .iter()
                .zip(my_exports.iter())
                .any(|((n1, t1), (n2, t2))| n1 != n2 && t1 != t2)
            {
                let mut export_section = enc::ExportSection::new();
                for (name, fnc) in my_exports {
                    export_section.export(&name, enc::Export::Function(fnc));
                }
            }
        }

        {
            let mut code_section = enc::CodeSection::new();

            for function in &self.function_bodies {
                code_section.function(&remap_function(function, &type_map, &function_map)?);
            }

            module.section(&code_section);
        }

        Ok(module.finish())
    }
}

fn copy_section(module: &mut wasm_encoder::Module, data: &[u8]) -> Result<()> {
    let mut reader = wasmparser::BinaryReader::new(data);
    let id = reader.read_u8()? as u8;
    let size = reader.read_var_u32()?;

    let data = &data[reader.current_position()..];
    assert!(data.len() == size as usize);

    module.section(&wasm_encoder::RawSection {
        id,
        data: &data[reader.current_position()..],
    });

    Ok(())
}

fn read_type_section(reader: TypeSectionReader) -> Result<Vec<base_module::FunctionType>> {
    let mut function_types = vec![];

    for type_def in reader {
        match type_def? {
            wasmparser::TypeDef::Func(fnc) => {
                if fnc.returns.len() > 1 {
                    bail!("Multi-value not supported");
                }
                let params = to_val_type_vec(&fnc.params)?;
                let result = to_val_type_vec(&fnc.returns)?.into_iter().next();
                function_types.push(FunctionType { params, result });
            }
            t => bail!("Unsupported type def {:?}", t),
        }
    }

    Ok(function_types)
}

#[derive(Debug)]
struct Section<T> {
    range: std::ops::Range<usize>,
    data: T,
}

impl<T> Section<T> {
    fn new(range: std::ops::Range<usize>, data: T) -> Section<T> {
        Section { range, data }
    }
}

#[derive(Debug)]
struct ImportSection {
    memory: u32,
    functions: Vec<FunctionImport>,
}

impl ImportSection {
    fn parse(reader: ImportSectionReader) -> Result<ImportSection> {
        let mut memory = 0;
        let mut functions = vec![];

        for import in reader {
            let import = import?;
            if let Some(field) = import.field {
                match import.ty {
                    ImportSectionEntryType::Function(type_) => {
                        functions.push(FunctionImport {
                            module: import.module.to_string(),
                            field: field.to_string(),
                            type_,
                        });
                    }
                    ImportSectionEntryType::Memory(mem) => {
                        if import.module != "env" || field != "memory" {
                            bail!(
                                "Wrong name of memory import {}.{}, should be env.memory",
                                import.module,
                                field
                            );
                        }
                        if mem.memory64 || mem.shared {
                            bail!("Wrong memory import options: {:?}", import.ty);
                        }
                        memory = mem.maximum.unwrap_or(mem.initial) as u32;
                    }
                    _ => bail!("Unsupported import item {:?}", import.ty),
                }
            } else {
                bail!(
                    "Found import without field, only module '{}'",
                    import.module
                );
            }
        }

        if memory == 0 {
            bail!("No memory import found");
        }

        Ok(ImportSection { memory, functions })
    }
}

#[derive(Debug)]
struct FunctionImport {
    module: String,
    field: String,
    type_: u32,
}

fn read_function_section(reader: FunctionSectionReader) -> Result<Vec<u32>> {
    let mut functions = vec![];
    for func_type in reader {
        functions.push(func_type?);
    }
    Ok(functions)
}

fn read_export_section(reader: ExportSectionReader) -> Result<Vec<(String, u32)>> {
    let mut function_exports = Vec::new();
    for export in reader {
        let export = export?;
        match export.kind {
            ExternalKind::Function => {
                function_exports.push((export.field.to_string(), export.index));
            }
            _ => bail!("Only function exports supported, found {:?}", export.kind),
        }
    }
    Ok(function_exports)
}

fn remap_function(
    reader: &FunctionBody,
    type_map: &HashMap<u32, u32>,
    function_map: &HashMap<u32, u32>,
) -> Result<enc::Function> {
    let mut locals = Vec::new();
    for local in reader.get_locals_reader()? {
        let (count, type_) = local?;
        locals.push((count, to_val_type(&type_)?));
    }
    let mut function = enc::Function::new(locals);

    let block_type = |ty: wasmparser::TypeOrFuncType| -> Result<enc::BlockType> {
        Ok(match ty {
            wasmparser::TypeOrFuncType::Type(wasmparser::Type::EmptyBlockType) => {
                enc::BlockType::Empty
            }
            wasmparser::TypeOrFuncType::Type(ty) => enc::BlockType::Result(to_val_type(&ty)?),
            wasmparser::TypeOrFuncType::FuncType(ty) => enc::BlockType::FunctionType(
                *type_map
                    .get(&ty)
                    .ok_or_else(|| anyhow!("Function type index out of range: {}", ty))?,
            ),
        })
    };

    fn mem(m: wasmparser::MemoryImmediate) -> enc::MemArg {
        enc::MemArg {
            offset: m.offset,
            align: m.align as u32,
            memory_index: m.memory,
        }
    }

    use enc::Instruction as En;
    use wasmparser::Operator as De;

    for op in reader.get_operators_reader()? {
        function.instruction(&match op? {
            De::Unreachable => En::Unreachable,
            De::Nop => En::Nop,
            De::Block { ty } => En::Block(block_type(ty)?),
            De::Loop { ty } => En::Loop(block_type(ty)?),
            De::If { ty } => En::If(block_type(ty)?),
            De::Else => En::Else,
            De::Try { .. } | De::Catch { .. } | De::Throw { .. } | De::Rethrow { .. } => todo!(),
            De::End => En::End,
            De::Br { relative_depth } => En::Br(relative_depth),
            De::BrIf { relative_depth } => En::BrIf(relative_depth),
            De::BrTable { .. } => todo!(),
            De::Return => En::Return,
            De::Call { function_index } => En::Call(
                dbg!(*function_map
                    .get(&function_index)
                    .ok_or_else(|| anyhow!("Function index out of range: {}", function_index))?,
            )),
            De::CallIndirect { .. }
            | De::ReturnCall { .. }
            | De::ReturnCallIndirect { .. }
            | De::Delegate { .. }
            | De::CatchAll => todo!(),
            De::Drop => En::Drop,
            De::Select => En::Select,
            De::TypedSelect { .. } => todo!(),
            De::LocalGet { local_index } => En::LocalGet(local_index),
            De::LocalSet { local_index } => En::LocalSet(local_index),
            De::LocalTee { local_index } => En::LocalTee(local_index),
            De::GlobalGet { global_index } => En::GlobalGet(global_index),
            De::GlobalSet { global_index } => En::GlobalSet(global_index),
            De::I32Load { memarg } => En::I32Load(mem(memarg)),
            De::I64Load { memarg } => En::I64Load(mem(memarg)),
            De::F32Load { memarg } => En::F32Load(mem(memarg)),
            De::F64Load { memarg } => En::F64Load(mem(memarg)),
            De::I32Load8S { memarg } => En::I32Load8_S(mem(memarg)),
            De::I32Load8U { memarg } => En::I32Load8_U(mem(memarg)),
            De::I32Load16S { memarg } => En::I32Load16_S(mem(memarg)),
            De::I32Load16U { memarg } => En::I32Load16_U(mem(memarg)),
            De::I64Load8S { memarg } => En::I64Load8_S(mem(memarg)),
            De::I64Load8U { memarg } => En::I64Load8_U(mem(memarg)),
            De::I64Load16S { memarg } => En::I64Load16_S(mem(memarg)),
            De::I64Load16U { memarg } => En::I64Load16_U(mem(memarg)),
            De::I64Load32S { memarg } => En::I64Load32_S(mem(memarg)),
            De::I64Load32U { memarg } => En::I64Load32_U(mem(memarg)),
            De::I32Store { memarg } => En::I32Store(mem(memarg)),
            De::I64Store { memarg } => En::I64Store(mem(memarg)),
            De::F32Store { memarg } => En::F32Store(mem(memarg)),
            De::F64Store { memarg } => En::F64Store(mem(memarg)),
            De::I32Store8 { memarg } => En::I32Store8(mem(memarg)),
            De::I32Store16 { memarg } => En::I32Store16(mem(memarg)),
            De::I64Store8 { memarg } => En::I64Store8(mem(memarg)),
            De::I64Store16 { memarg } => En::I64Store16(mem(memarg)),
            De::I64Store32 { memarg } => En::I64Store32(mem(memarg)),
            De::MemorySize { mem, mem_byte: _ } => En::MemorySize(mem),
            De::MemoryGrow { mem, mem_byte: _ } => En::MemoryGrow(mem),
            De::I32Const { value } => En::I32Const(value),
            De::I64Const { value } => En::I64Const(value),
            De::F32Const { value } => En::F32Const(f32::from_bits(value.bits())),
            De::F64Const { value } => En::F64Const(f64::from_bits(value.bits())),
            De::RefNull { .. } | De::RefIsNull { .. } | De::RefFunc { .. } => todo!(),
            De::I32Eqz => En::I32Eqz,
            De::I32Eq => En::I32Eq,
            De::I32Ne => En::I32Neq,
            De::I32LtS => En::I32LtS,
            De::I32LtU => En::I32LtU,
            De::I32GtS => En::I32GtS,
            De::I32GtU => En::I32GtU,
            De::I32LeS => En::I32LeS,
            De::I32LeU => En::I32LeU,
            De::I32GeS => En::I32GeS,
            De::I32GeU => En::I32GeU,
            De::I64Eqz => En::I64Eqz,
            De::I64Eq => En::I64Eq,
            De::I64Ne => En::I64Neq,
            De::I64LtS => En::I64LtS,
            De::I64LtU => En::I64LtU,
            De::I64GtS => En::I64GtS,
            De::I64GtU => En::I64GtU,
            De::I64LeS => En::I64LeS,
            De::I64LeU => En::I64LeU,
            De::I64GeS => En::I64GeS,
            De::I64GeU => En::I64GeU,
            De::F32Eq => En::F32Eq,
            De::F32Ne => En::F32Neq,
            De::F32Lt => En::F32Lt,
            De::F32Gt => En::F32Gt,
            De::F32Le => En::F32Le,
            De::F32Ge => En::F32Ge,
            De::F64Eq => En::F64Eq,
            De::F64Ne => En::F64Neq,
            De::F64Lt => En::F64Lt,
            De::F64Gt => En::F64Gt,
            De::F64Le => En::F64Le,
            De::F64Ge => En::F64Ge,
            De::I32Clz => En::I32Clz,
            De::I32Ctz => En::I32Ctz,
            De::I32Popcnt => En::I32Popcnt,
            De::I32Add => En::I32Add,
            De::I32Sub => En::I32Sub,
            De::I32Mul => En::I32Mul,
            De::I32DivS => En::I32DivS,
            De::I32DivU => En::I32DivU,
            De::I32RemS => En::I32RemS,
            De::I32RemU => En::I32RemU,
            De::I32And => En::I32And,
            De::I32Or => En::I32Or,
            De::I32Xor => En::I32Xor,
            De::I32Shl => En::I32Shl,
            De::I32ShrS => En::I32ShrS,
            De::I32ShrU => En::I32ShrU,
            De::I32Rotl => En::I32Rotl,
            De::I32Rotr => En::I32Rotr,
            De::I64Clz => En::I64Clz,
            De::I64Ctz => En::I64Ctz,
            De::I64Popcnt => En::I64Popcnt,
            De::I64Add => En::I64Add,
            De::I64Sub => En::I64Sub,
            De::I64Mul => En::I64Mul,
            De::I64DivS => En::I64DivS,
            De::I64DivU => En::I64DivU,
            De::I64RemS => En::I64RemS,
            De::I64RemU => En::I64RemU,
            De::I64And => En::I64And,
            De::I64Or => En::I64Or,
            De::I64Xor => En::I64Xor,
            De::I64Shl => En::I64Shl,
            De::I64ShrS => En::I64ShrS,
            De::I64ShrU => En::I64ShrU,
            De::I64Rotl => En::I64Rotl,
            De::I64Rotr => En::I64Rotr,
            De::F32Abs => En::F32Abs,
            De::F32Neg => En::F32Neg,
            De::F32Ceil => En::F32Ceil,
            De::F32Floor => En::F32Floor,
            De::F32Trunc => En::F32Trunc,
            De::F32Nearest => En::F32Nearest,
            De::F32Sqrt => En::F32Sqrt,
            De::F32Add => En::F32Add,
            De::F32Sub => En::F32Sub,
            De::F32Mul => En::F32Mul,
            De::F32Div => En::F32Div,
            De::F32Min => En::F32Min,
            De::F32Max => En::F32Max,
            De::F32Copysign => En::F32Copysign,
            De::F64Abs => En::F64Abs,
            De::F64Neg => En::F64Neg,
            De::F64Ceil => En::F64Ceil,
            De::F64Floor => En::F64Floor,
            De::F64Trunc => En::F64Trunc,
            De::F64Nearest => En::F64Nearest,
            De::F64Sqrt => En::F64Sqrt,
            De::F64Add => En::F64Add,
            De::F64Sub => En::F64Sub,
            De::F64Mul => En::F64Mul,
            De::F64Div => En::F64Div,
            De::F64Min => En::F64Min,
            De::F64Max => En::F64Max,
            De::F64Copysign => En::F64Copysign,
            De::I32WrapI64 => En::I32WrapI64,
            De::I32TruncF32S => En::I32TruncF32S,
            De::I32TruncF32U => En::I32TruncF32U,
            De::I32TruncF64S => En::I32TruncF64S,
            De::I32TruncF64U => En::I32TruncF64U,
            De::I64ExtendI32S => En::I64ExtendI32S,
            De::I64ExtendI32U => En::I64ExtendI32U,
            De::I64TruncF32S => En::I64TruncF32S,
            De::I64TruncF32U => En::I64TruncF32U,
            De::I64TruncF64S => En::I64TruncF64S,
            De::I64TruncF64U => En::I64TruncF64U,
            De::F32ConvertI32S => En::F32ConvertI32S,
            De::F32ConvertI32U => En::F32ConvertI32U,
            De::F32ConvertI64S => En::F32ConvertI64S,
            De::F32ConvertI64U => En::F32ConvertI64U,
            De::F32DemoteF64 => En::F32DemoteF64,
            De::F64ConvertI32S => En::F64ConvertI32S,
            De::F64ConvertI32U => En::F64ConvertI32U,
            De::F64ConvertI64S => En::F64ConvertI64S,
            De::F64ConvertI64U => En::F64ConvertI64U,
            De::F64PromoteF32 => En::F64PromoteF32,
            De::I32ReinterpretF32 => En::I32ReinterpretF32,
            De::I64ReinterpretF64 => En::I64ReinterpretF64,
            De::F32ReinterpretI32 => En::F32ReinterpretI32,
            De::F64ReinterpretI64 => En::F64ReinterpretI64,
            De::I32Extend8S => En::I32Extend8S,
            De::I32Extend16S => En::I32Extend16S,
            De::I64Extend8S => En::I64Extend8S,
            De::I64Extend16S => En::I64Extend16S,
            De::I64Extend32S => En::I64Extend32S,
            De::I32TruncSatF32S => En::I32TruncSatF32S,
            De::I32TruncSatF32U => En::I32TruncSatF32U,
            De::I32TruncSatF64S => En::I32TruncSatF64S,
            De::I32TruncSatF64U => En::I32TruncSatF64U,
            De::I64TruncSatF32S => En::I64TruncSatF32S,
            De::I64TruncSatF32U => En::I64TruncSatF32U,
            De::I64TruncSatF64S => En::I64TruncSatF64S,
            De::I64TruncSatF64U => En::I64TruncSatF64U,
            other => bail!("Unsupported instruction {:?}", other),
        });
    }

    Ok(function)
}
