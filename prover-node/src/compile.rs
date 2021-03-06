
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status::NotFound;
use serde_json::to_writer_pretty;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};
use typed_arena::Arena;
use zokrates_common::constants::BN128;
use zokrates_common::helpers::CurveParameter;
use zokrates_core::compile::{compile, CompileConfig, CompileError};
use zokrates_field::{Bn128Field, Field};
use zokrates_fs_resolver::FileSystemResolver;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CompileRequestBody {
    program: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CompileResponseBody {}

#[post("/compile", data = "<task>", format = "json")]
pub fn post_compile_zokrates(
    task: Json<CompileRequestBody>
) -> Result<Json<CompileResponseBody>, NotFound<String>> { 
    match api_compile::<Bn128Field>() {
        Ok(_) => Ok(Json(CompileResponseBody {})),
        Err(str) => Err(NotFound(str)),
    }
}

fn api_compile<T: Field>() -> Result<(), String> {
    println!("Compiling proving/proof_of_ownership.zok\n");
    let path = PathBuf::from("proving/proof_of_ownership.zok");
    let bin_output_path = Path::new("out/compile_out");
    let abi_spec_path = Path::new("abi.json");

    log::debug!("Load entry point file {}", path.display());

    let file = File::open(path.clone())
        .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

    let mut reader = BufReader::new(file);
    let mut source = String::new();
    reader.read_to_string(&mut source).unwrap();

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

    let stdlib_path = "/home/zokrates/.zokrates/stdlib]";
    match Path::new(stdlib_path).exists() {
        true => Ok(()),
        _ => Err(format!(
            "Invalid standard library source path: {}",
            stdlib_path
        )),
    }?;

    let config =
        CompileConfig::default();

    let resolver =  FileSystemResolver::with_stdlib_root(stdlib_path);

    log::debug!("Compile");

    let arena = Arena::new();

    let artifacts =
        compile::<T, _>(source, path, Some(&resolver), config, &arena).map_err(|e| {
            format!(
                "Compilation failed:\n\n{}",
                e.0.iter()
                    .map(|e| fmt_error(e))
                    .collect::<Vec<_>>()
                    .join("\n\n")
            )
        })?;

    let (program_flattened, abi) = artifacts.into_inner();

    // serialize flattened program and write to binary file
    log::debug!("Serialize program");
    let bin_output_file = File::create(&bin_output_path)
        .map_err(|why| format!("Could not create {}: {}", bin_output_path.display(), why))?;

    let mut writer = BufWriter::new(bin_output_file);

    match program_flattened.serialize(&mut writer) {
        Ok(constraint_count) => {
            // serialize ABI spec and write to JSON file
            log::debug!("Serialize ABI");
            let abi_spec_file = File::create(&abi_spec_path)
                .map_err(|why| format!("Could not create {}: {}", abi_spec_path.display(), why))?;

            let mut writer = BufWriter::new(abi_spec_file);
            to_writer_pretty(&mut writer, &abi)
                .map_err(|_| "Unable to write data to file.".to_string())?;

            println!("Compiled code written to '{}'", bin_output_path.display());

            println!("Number of constraints: {}", constraint_count);

            Ok(())
        }
        Err(e) => {
            // something wrong happened, clean up
            std::fs::remove_file(bin_output_path).unwrap();
            Err(e.to_string())
        }
    }
}

// FIXME: add unittest for route
// #[cfg(test)] use rocket::local::blocking::Client;
// #[cfg(test)] use rocket::http::{Status, ContentType};

// mock generate_proof function
//  #[test]
// fn test_post_generate_proof() {
//     let client = Client::tracked(super::rocket()).unwrap();
//     let res = client.post("/generate-proof")
//         .header(ContentType::JSON)
//         .body(r##"{
//             "proving_key": "ridicolous text"
//         }"##)
//         .dispatch();
//     assert_eq!(res.status(), Status::Ok);
// }

//  #[test]
// fn test_generate_proof() {
//     let proof = let proof = generate_proof::<_, _, GM17, Ark>(p)
                // .map_err(|e| NotFound(e.to_string()))?;
//     assert_eq!(proof, blablabla);
// }