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