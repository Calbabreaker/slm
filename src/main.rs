use std::{path::Path, process::Command};

fn compile(file: &Path) -> anyhow::Result<()> {
    let source = slm::Source::from_file(Path::new(&file))?;
    let asm = slm::compile(&source).map_err(|e| e.with_source(source))?;
    println!("{asm}");

    let nasm_file = file.with_extension("asm");
    let object_file = file.with_extension("o");
    let exe_file = file.with_extension("");

    std::fs::write(&nasm_file, asm)?;
    Command::new("nasm")
        .arg("-felf64")
        .arg(nasm_file)
        .arg("-o")
        .arg(&object_file)
        .spawn()?;

    Command::new("ld")
        .arg(&object_file)
        .arg("-o")
        .arg(&exe_file)
        .spawn()?;

    Ok(())
}

fn main() {
    let mut args = std::env::args();
    let file = args.nth(1).unwrap_or_else(|| "".into());
    compile(Path::new(&file))
        .inspect_err(|e| eprintln!("{e}"))
        .ok();
}
