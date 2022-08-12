use zokrates_core::ir;
use zokrates_field::Field;
use serde_json::to_string;
use zokrates_abi::Encode;
use zokrates_core::typed_absy::abi::Abi;
use zokrates_abi::Inputs;


pub fn compute_witness<T: Field, I: Iterator<Item = ir::Statement<T>>>(
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


#[cfg(test)]
mod test {
    use super::*;
    use zokrates_core::ir::ProgEnum;
    use std::fs::File;
    use std::io::BufReader;

    const ABI: &str = r#"{
            "inputs": [
                {
                    "name": "N",
                    "public": true,
                    "type": "field"
                }
            ],
            "outputs": [
                {
                    "type": "bool"
                }
            ]
        }"#;

    #[test]
    fn test_correct_witness_computation() {
        let file = File::open("tests/test").unwrap();
        let mut reader = BufReader::new(file);
        let prog = ProgEnum::deserialize(&mut reader).unwrap();
        let witness_args = serde_json::to_value(["1"]).unwrap();
        let abi = serde_json::from_str(ABI).unwrap();

        let witness = match prog {
            ProgEnum::Bn128Program(p) => compute_witness(p, witness_args, abi),
            _ => unreachable!(),
        };
        assert!(witness.is_ok());

        let (_, output) = witness.unwrap();
        assert_eq!(output[0], true);
    }

    #[test]
    fn test_wrong_witness_computation() {
        let file = File::open("tests/test").unwrap();
        let mut reader = BufReader::new(file);
        let prog = ProgEnum::deserialize(&mut reader).unwrap();
        let witness_args = serde_json::to_value(["2"]).unwrap();
        let abi = serde_json::from_str(ABI).unwrap();

        let witness = match prog {
            ProgEnum::Bn128Program(p) => compute_witness(p, witness_args, abi),
            _ => unreachable!(),
        };
        assert!(witness.is_ok());

        let (witness_out, output) = witness.unwrap();
        assert_eq!(output[0], false);
        println!("{}", witness_out);
    }

    #[test]
    fn test_witness_computation_wrong_abi() {
        let file = File::open("tests/test").unwrap();
        let mut reader = BufReader::new(file);
        let prog = ProgEnum::deserialize(&mut reader).unwrap();
        let witness_args = serde_json::to_value(["abcd", "2"]).unwrap();
        let abi = serde_json::from_str(ABI).unwrap();

        let witness = match prog {
            ProgEnum::Bn128Program(p) => compute_witness(p, witness_args, abi),
            _ => unreachable!(),
        };
        assert!(witness.is_err());
    }
}