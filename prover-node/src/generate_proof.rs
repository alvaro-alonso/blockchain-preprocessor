
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status::NotFound;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use zokrates_core::ir;
use zokrates_core::ir::ProgEnum;
use zokrates_core::proof_system::*;
use zokrates_field::Field;
// #[cfg(feature = "ark")]
use zokrates_core::proof_system::ark::Ark;
use zokrates_core::proof_system::GM17;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofRequestBody {
    proving_key: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GenerateProofResponseBody {}

#[post("/", data = "<task>", format = "json")]
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
            cli_generate_proof::<_, _, GM17, Ark>(p)
                .map_err(|e| NotFound(e.to_string()))?;
        }
        _ => unreachable!(),
    }
    
    Ok(Json(GenerateProofResponseBody {}))

 }

fn cli_generate_proof<
    T: Field,
    I: Iterator<Item = ir::Statement<T>>,
    S: Scheme<T>,
    B: Backend<T, S>
>(
    program: ir::ProgIterator<T, I>,
) -> Result<(), String> {
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
    let proof_path = Path::new("proving/trial.json");
    let pk_file = File::open(&pk_path)
        .map_err(|why| format!("Could not open {}: {}", pk_path.display(), why))?;

    let mut pk: Vec<u8> = Vec::new();
    let mut pk_reader = BufReader::new(pk_file);
    pk_reader
        .read_to_end(&mut pk)
        .map_err(|why| format!("Could not read {}: {}", pk_path.display(), why))?;
    println!("read proving key successfully");

    let proof = B::generate_proof(program, witness, pk);
    let mut proof_file = File::create(proof_path).unwrap();

    let proof =
        serde_json::to_string_pretty(&TaggedProof::<T, S>::new(proof.proof, proof.inputs)).unwrap();
    proof_file
        .write(proof.as_bytes())
        .map_err(|why| format!("Could not write to {}: {}", proof_path.display(), why))?;

    println!("Proof:\n{}", proof);

    println!("Proof written to '{}'", proof_path.display());
    Ok(())
}

use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};

 #[test]
fn test_post_generate_proof() {
    let client = Client::tracked(super::rocket()).unwrap();
    let res = client.post("/generate-proof")
        .header(ContentType::JSON)
        .body(r##"{
            "proving_key": "ridicolous text"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}