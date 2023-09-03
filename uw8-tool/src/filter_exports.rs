use anyhow::Result;
use std::path::Path;

pub fn filter_exports(in_path: &Path, out_path: &Path) -> Result<()> {
    let mut module = walrus::Module::from_file(in_path)?;

    let exports_to_delete: Vec<_> = module
        .exports
        .iter()
        .filter_map(|export| match export.name.as_str() {
            "start" | "upd" | "snd" => None,
            _ => Some(export.id()),
        })
        .collect();

    for id in exports_to_delete {
        module.exports.delete(id);
    }

    walrus::passes::gc::run(&mut module);

    module.emit_wasm_file(out_path)?;

    Ok(())
}
