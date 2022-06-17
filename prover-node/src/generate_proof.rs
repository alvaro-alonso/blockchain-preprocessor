
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status::NotFound;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zokrates_core::ir;
use zokrates_core::ir::ProgEnum;
use zokrates_core::proof_system::*;
use zokrates_field::Field;
use zokrates_core::proof_system::ark::Ark;
use zokrates_core::proof_system::GM17;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofRequestBody {
    proving_key: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofResponseBody {
    // TODO: serialize TaggedProof
    payload: serde_json::Value,
}

#[post("/generate-proof", data = "<task>", format = "json")]
pub fn post_generate_proof(
    task: Json<GenerateProofRequestBody>
) -> Result<Json<GenerateProofResponseBody>, NotFound<String>> { 
    // parse input program
    let program_path = Path::new("proving/proof_of_ownership");
    println!("hello!");
    let program_file = File::open(&program_path)
        .map_err(|e| NotFound(e.to_string()))?;
    println!("file read successfully");
    let mut reader = BufReader::new(program_file);
    let prog = ProgEnum::deserialize(&mut reader)
        .map_err(|e| NotFound(e.to_string()))?;
    println!("deserialization successfull");

    // #[cfg(feature = "ark")]
    match prog {
        ProgEnum::Bn128Program(p) => {
            let proof = generate_proof::<_, _, GM17, Ark>(p)
                .map_err(|e| NotFound(e.to_string()))?;

            let proof_str =
                serde_json::to_string_pretty(&proof).unwrap();
            println!("Proof:\n{}", proof_str);
            let proof = serde_json::from_str(&proof_str).unwrap();
            
            Ok(Json(GenerateProofResponseBody {
                payload: proof,
            }))
        }
        _ => unreachable!(),
    }
 }

fn generate_proof<
    T: Field,
    I: Iterator<Item = ir::Statement<T>>,
    S: Scheme<T>,
    B: Backend<T, S>
>(
    program: ir::ProgIterator<T, I>,
) -> Result<TaggedProof<T, S>, String> {
    println!("Generating proof...");
    
    // read witness
    let witness_path = Path::new("proving/witness");
    let witness_file = File::open(&witness_path)
        .map_err(|why| format!("Could not open {}: {}", witness_path.display(), why))?;
    let witness = ir::Witness::read(witness_file)
        .map_err(|why| format!("Could not load witness: {:?}", why))?;
    println!("read witness successfully");

    // read proving key
    let pk_path = Path::new("proving/proving.key");
    let pk_file = File::open(&pk_path)
        .map_err(|why| format!("Could not open {}: {}", pk_path.display(), why))?;

    let mut pk: Vec<u8> = Vec::new();
    let mut pk_reader = BufReader::new(pk_file);
    pk_reader
        .read_to_end(&mut pk)
        .map_err(|why| format!("Could not read {}: {}", pk_path.display(), why))?;
    println!("read proving key successfully");

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