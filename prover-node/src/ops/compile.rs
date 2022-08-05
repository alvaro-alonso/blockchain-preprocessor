use std::path::{Path, PathBuf};
use typed_arena::Arena;
use zokrates_core::compile::{compile, CompileConfig, CompileError};
use zokrates_field::Field;
use zokrates_fs_resolver::FileSystemResolver;


pub fn api_compile<'a, T: Field>(code: &'a str, program_path: &'a PathBuf, arena: &'a Arena<String>) 
-> Result<(
    zokrates_core::ir::ProgIterator<T, impl std::iter::IntoIterator<Item = zokrates_core::ir::Statement<T>> + 'a>,
    zokrates_core::typed_absy::abi::Abi
), String> {

    let fmt_error = |e: &CompileError| {
        let file = e.file().canonicalize().unwrap();
        format!(
            "{}:{}",
            file.strip_prefix(std::env::current_dir().unwrap())
                .unwrap_or(file.as_path())
                .display(),
            e.value()
        )
    };

    let stdlib_path = "zokrates/zokrates_stdlib/stdlib";
    match Path::new(stdlib_path).exists() {
        true => Ok(()),
        _ => Err(format!(
            "Invalid standard library source path: {}",
            stdlib_path
        )),
    }?;

    let config = CompileConfig::default();
    let resolver = FileSystemResolver::with_stdlib_root(stdlib_path);
    log::debug!("Compile");

    let program = code.to_string();
    // FIXME: compile error cause api to panic
    match compile::<T, _>(program.clone(), program_path.clone(), Some(&resolver), config, &arena) {
        Ok(artifacts) => Ok(artifacts.into_inner()),
        Err(e) => Err(format!(
            "Compilation failed:\n\n{}",
            e.0.iter()
                .map(|e| fmt_error(e))
                .collect::<Vec<_>>()
                .join("\n\n")
        )),
    }    
}