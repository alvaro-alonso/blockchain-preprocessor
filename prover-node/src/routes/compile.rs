use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::fs::{relative};
use rocket::http::Status;
use serde_json::to_writer_pretty;
use std::fs::{File, create_dir, write, remove_dir_all};
use std::io::BufWriter;
use std::path::Path;
use typed_arena::Arena;
use sha2::{Sha256, Digest};
use zokrates_core::typed_absy::abi::Abi;
use zokrates_field::Bn128Field;
use prover_node::ops::compile::api_compile;
use prover_node::utils::responses::{ApiResult, ApiResponse, ApiError};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CompileRequestBody {
    program: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CompileResponseBody {
    proof_id: String,
    abi: Abi,
}

#[post("/compile", data = "<req_body>", format = "json")]
pub fn post_compile_zokrates(
    req_body: Json<CompileRequestBody>,
) -> ApiResult<CompileResponseBody> {
    // create a hash for the .zok code, if the hash exists return err
    let program = req_body.program.clone();
    let hash = format!("{:X}", Sha256::digest(&program));
    let path = Path::new(relative!("out")).join(&hash);
    if path.is_dir() {
        return Err(ApiError::ResourceAlreadyExists(String::from("proof already exists")))
    } 

    // create all file paths
    let program_path = path.join("program.zok");
    let bin_output_path = path.join("out");
    let abi_spec_path = path.join("abi.json");
    let arena = Arena::new();

    // compile .zok code
    let (program_flattened, abi) = api_compile::<Bn128Field>(&program, &program_path, &arena)
        .map_err(|e| ApiError::CompilationError(e))?;
        
    // if compilation successful write .zok, binary and abi file under the hash folder
    let write_outputs = || -> Result<usize, String> {
        // create dir with the hash of the program
        create_dir(&path).map_err(|e| e.to_string())?;

        // serialize flattened program and write to binary file
        log::debug!("Serialize program");
        let bin_output_file = File::create(&bin_output_path)
            .map_err(|why| format!("Could not create {}: {}", bin_output_path.display(), why))?;
        let mut writer = BufWriter::new(bin_output_file);
        let constrain_count = program_flattened.serialize(&mut writer)
            .map_err(|e| e.to_string())?;

        // serialize ABI spec and write to JSON file
        log::debug!("Serialize ABI");
        let abi_spec_file = File::create(&abi_spec_path)
            .map_err(|why| format!("Could not create {}: {}", abi_spec_path.display(), why))?;
        let mut writer = BufWriter::new(abi_spec_file);
        to_writer_pretty(&mut writer, &abi)
            .map_err(|_| "Unable to write data to file.".to_string())?;

        // write .zok file in folder
        write(&program_path, &program).expect("Unable to write .zok file");
    
        Ok(constrain_count)
    };
    
    match write_outputs() {
        Ok(constrain_count) => {
            log::info!("zokrates program written to '{}'", program_path.display());
            log::info!("Compiled code written to '{}'", bin_output_path.display());
            log::info!("abi file written to '{}'", abi_spec_path.display());
            log::info!("Number of constraints: {}", constrain_count);
            Ok(ApiResponse {
                response: CompileResponseBody {
                    proof_id: hash,
                    abi,
                },
                status: Status::Created,
            })
        },
        Err(e) => {
            // something wrong happened, clean up
            remove_dir_all(path).unwrap();
            Err(ApiError::InternalError(e.to_string()))
        },
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