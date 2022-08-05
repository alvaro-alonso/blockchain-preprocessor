use rocket::post;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::fs::relative;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zokrates_core::ir;
use zokrates_core::ir::ProgEnum;
use zokrates_core::proof_system::ark::Ark;
use zokrates_core::proof_system::GM17;
use zokrates_core::proof_system::*;
use zokrates_field::Field;
use prover_node::utils::responses::{ApiResult, ApiResponse, ApiError};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofRequestBody {
    witness: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofResponseBody {
    // TODO: serialize TaggedProof
    payload: serde_json::Value,
}

// TODO: add generate proof from request arguments
#[post("/<hash>/generate-proof", format = "json", data = "<req_body>")] 
pub fn post_generate_proof(hash: &str, req_body: Json<GenerateProofRequestBody>) -> ApiResult<GenerateProofResponseBody> {
    // parse input program
    let program_dir = Path::new(relative!("out")).join(&hash);
    if !program_dir.is_dir() {
        return Err(ApiError::ResourceNotFound(format!("Proof {} have not been registered", hash)))
    }

    // read binary file
    let mut path = program_dir.join("out");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!("Binary file for proof {} does not exists. Commile the program first", hash)))
    }
    let program_file = File::open(&path).map_err(|e| ApiError::InternalError(e.to_string()))?;
    let mut reader = BufReader::new(program_file);
    let prog = ProgEnum::deserialize(&mut reader).map_err(|e| ApiError::InternalError(e.to_string()))?;
    log::debug!("binary deserialization successfull");

    // read proving key
    path = program_dir.join("proving.key");
    if !path.exists() {
        return Err(ApiError::ResourceNotFound(format!("Binary file for proof {} does not exists. Commile the program first", hash)))
    }
    let pk_file = File::open(&path)
        .map_err(|why| ApiError::InternalError(format!("Could not open {}: {}", path.display(), why)))?;
    let mut pk: Vec<u8> = Vec::new();
    let mut pk_reader = BufReader::new(pk_file);
    pk_reader
        .read_to_end(&mut pk)
        .map_err(|why| ApiError::InternalError(format!("Could not read {}: {}", path.display(), why)))?;
    log::debug!("read proving key successfully");

    // read witness for request body
    let witness = ir::Witness::read(req_body.witness.as_bytes())
        .map_err(|why| ApiError::InternalError(format!("Could not load witness: {:?}", why)))?;
    log::debug!("read witness successfully");

    

    match prog {
        ProgEnum::Bn128Program(p) => {
            let proof =
                generate_proof::<_, _, GM17, Ark>(p, witness, pk).map_err(|e| ApiError::CompilationError(e.to_string()))?;

            let proof_str = serde_json::to_string_pretty(&proof).unwrap();
            log::debug!("Proof:\n{}", proof_str);
            let proof = serde_json::from_str(&proof_str).unwrap();

            Ok(ApiResponse {
                response: GenerateProofResponseBody { payload: proof },
                status: Status::Accepted,
            })
        }
        _ => unreachable!(),
    }
}

fn generate_proof<
    T: Field,
    I: Iterator<Item = ir::Statement<T>>,
    S: Scheme<T>,
    B: Backend<T, S>,
>(
    program: ir::ProgIterator<T, I>,
    witness: zokrates_core::ir::Witness<T>,
    pk: std::vec::Vec<u8>,
) -> Result<TaggedProof<T, S>, String> {
    log::info!("Generating proof...");
    let proof = B::generate_proof(program, witness, pk);
    Ok(TaggedProof::<T, S>::new(proof.proof, proof.inputs))
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
