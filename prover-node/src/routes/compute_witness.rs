use rocket::post;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::fs::relative;
use rocket::http::Status;
use std::fs::File;
use std::path::{Path,PathBuf};
use zokrates_core::ir;
use zokrates_core::ir::ProgEnum;
use zokrates_field::Field;
use serde_json::{from_reader, to_string};
use std::io::{BufReader};
use zokrates_abi::Encode;
use zokrates_core::typed_absy::abi::Abi;
use zokrates_abi::Inputs;
use prover_node::utils::responses::{ApiResult, ApiResponse, ApiError};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WitnessRequestBody {
    payload: serde_json::Value,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WitnessResponseBody {
    output: serde_json::Value,
    witness: String,
}

#[post("/<hash>/compute-witness", data = "<witness>", format = "json")] //
pub fn post_witness(hash: &str, witness: Json<WitnessRequestBody>) -> ApiResult<WitnessResponseBody> {
    // parse input program
    let program_dir = Path::new(relative!("out")).join(&hash);
    if !program_dir.is_dir() {
        return Err(ApiError::ResourceNotFound(format!("Proof {} have not been registered", hash)))
    }

    //TODO: make file reading async
    // read binary file
    let mut path = program_dir.join("out");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!("Binary file for proof {} does not exists. Commile the program first", hash)))
    }
    let mut file = File::open(&path)
        .map_err(|why| ApiError::InternalError(format!("Could not open {}: {}", program_dir.display(), why)))?;
    let mut reader = BufReader::new(file);
    let prog = ProgEnum::deserialize(&mut reader).map_err(|why| ApiError::InternalError(why.to_string()))?;

    // read abi file
    path = program_dir.join("abi.json");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!("ABI file for proof {} does not exists. Commile the program first", hash)))
    }
    file = File::open(&path)
        .map_err(|why| ApiError::InternalError(format!("Could not open {}: {}", path.display(), why)))?;
    let mut reader = BufReader::new(file);
    let abi: Abi = from_reader(&mut reader).map_err(|why| ApiError::InternalError(why.to_string()))?;

    match prog {
        ProgEnum::Bn128Program(p) => {
            match compute_witness(p, witness.payload.clone(), abi){
                Ok((witness, output)) => Ok(ApiResponse {
                    response: WitnessResponseBody {
                        witness: witness.to_string(),
                        output,
                    },
                    status: Status::Created,
                }),
                Err(err) => Err(ApiError::CompilationError(format!("error computing witness:\n {}", err))),
            }
            
        },
        _ => unreachable!(),
    }
}

fn compute_witness<T: Field, I: Iterator<Item = ir::Statement<T>>>(
    ir_prog: ir::ProgIterator<T, I>,
    arguments: serde_json::Value,
    abi: Abi,
) -> Result<(zokrates_core::ir::Witness<T>, serde_json::Value), String> {
    log::info!("Computing witness...");
    let signature = abi.signature();

    // get arguments
    let input =  match to_string(&arguments) {
        Ok(args) => {
            use zokrates_abi::parse_strict;

            parse_strict(&args, signature.inputs)
                .map(Inputs::Abi)
                .map_err(|why| why.to_string())
        }
        Err(_) => Err(String::from("???")),
    }
    .map_err(|e| format!("Could not parse argument: {}", e))?;

    let interpreter = ir::Interpreter::default();

    let witness = interpreter
        .execute(ir_prog, &input.encode())
        .map_err(|e| format!("Execution failed: {}", e))?;

    use zokrates_abi::Decode;

    let results_json_value: serde_json::Value =
        zokrates_abi::Values::decode(witness.return_values(), signature.outputs).into_serde_json();

    log::debug!("\nWitness: \n{}\n", results_json_value);
    Ok((witness, results_json_value))

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
