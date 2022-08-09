use std::path::{Path, PathBuf};
use typed_arena::Arena;
use zokrates_core::ir::Statement;
use zokrates_core::compile::{compile, CompileConfig, CompilationArtifacts};
use zokrates_field::Field;
use zokrates_fs_resolver::FileSystemResolver;


pub fn api_compile<'a, T: Field>(code: &'a str, program_path: &'a PathBuf, arena: &'a Arena<String>) 
-> Result<
    CompilationArtifacts<T, impl IntoIterator<Item = Statement<T>> + 'a>,
    String
> {

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
    match compile::<T, _>(program.clone(), program_path.clone(), Some(&resolver), config, &arena) {
        Ok(artifacts) => Ok(artifacts),
        Err(e) => Err(format!(
            "Compilation failed:\n\n{}",
            e.0.iter()
                .map(|e| format!("{}", e.value()))
                .collect::<Vec<_>>()
                .join("\n\n")
        )),
    }    
}

#[cfg(test)]
mod test {
    use super::*;
    use zokrates_field::Bn128Field;
    use std::io::stdout;

    #[test]
    fn test_sucessful_compilation() {
        let code = r#"
            def main(field N) -> (bool):
                assert(N == 1)
                return true
        "#;
        let code_path = PathBuf::from("/test");
        let arena = Arena::new();

        let compilation = api_compile::<Bn128Field>(&code, &code_path, &arena);
        assert!(compilation.is_ok());

        let (compiled_program, _abi) = compilation.unwrap().into_inner();
        let constrain_count = compiled_program.serialize(&stdout()).unwrap();
        assert_eq!(constrain_count, 2);

        //TODO: assert that abi is equal to:
        //   {
        //     "inputs": [
        //       {
        //         "name": "N",
        //         "public": true,
        //         "type": "field"
        //       }
        //     ],
        //     "outputs": [
        //       {
        //         "type": "bool"
        //       }
        //     ]
        //   }
    }

        #[test]
    fn test_wrong_compilation() {
        let code = r#"
            def main(field N):
                assert(N == 1)
                return true
        "#;
        let code_path = PathBuf::from("/test");
        let arena = Arena::new();

        let compilation = api_compile::<Bn128Field>(&code, &code_path, &arena);
        assert!(compilation.is_err());

        //TODO: assert that error types are the same
    }
}