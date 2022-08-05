use rocket::post;
use rocket::response::status::NotFound;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use zokrates_core::ir;
use zokrates_core::ir::ProgEnum;
use zokrates_field::Field;
use serde_json::{from_reader, to_string};
use std::io::{BufReader, BufWriter};
use zokrates_abi::Encode;
use zokrates_core::typed_absy::abi::Abi;
use zokrates_abi::Inputs;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WitnessRequestBody {
    payload: serde_json::Value,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WitnessResponseBody {
    message: String,
}

// TODO: add generate proof from request arguments
#[post("/compute-witness", data = "<witness>", format = "json")] //
pub fn post_witness(
    witness: Json<WitnessRequestBody>,
) -> Result<Json<WitnessResponseBody>, NotFound<String>> {
    // parse input program
    let path = Path::new("proving/proof_of_ownership");
    let file =
        File::open(&path).map_err(|why| NotFound(format!("Could not open {}: {}", path.display(), why)))?;

    let mut reader = BufReader::new(file);
    let prog = ProgEnum::deserialize(&mut reader).map_err(|why| NotFound(why.to_string()))?;

    match prog {
        ProgEnum::Bn128Program(p) => {
            let _comp = cli_compute(p, witness.payload.clone());
            Ok(Json(WitnessResponseBody { message: String::from("Ok") }))
        },
        _ => unreachable!(),
    }
}

fn cli_compute<T: Field, I: Iterator<Item = ir::Statement<T>>>(
    ir_prog: ir::ProgIterator<T, I>,
    arguments: serde_json::Value,
) -> Result<(), String> {
    log::info!("Computing witness...");

    let is_stdin = true;
    let is_abi = true;

    if !is_stdin && is_abi {
        return Err("ABI input as inline argument is not supported. Please use `--stdin`.".into());
    }


    let path = Path::new("proving/abi.json");
    let file = File::open(&path)
        .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;
    let mut reader = BufReader::new(file);

    let abi: Abi = from_reader(&mut reader).map_err(|why| why.to_string())?;

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

    log::info!("\nWitness: \n{}\n", results_json_value);

    // write witness to file
    let output_path = Path::new("out/witness");
    let output_file = File::create(&output_path)
        .map_err(|why| format!("Could not create {}: {}", output_path.display(), why))?;

    let writer = BufWriter::new(output_file);

    witness
        .write(writer)
        .map_err(|why| format!("Could not save witness: {:?}", why))?;

    log::info!("Witness file written to '{}'", output_path.display());
    Ok(())
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
