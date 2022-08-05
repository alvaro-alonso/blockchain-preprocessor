use zokrates_core::ir;
use zokrates_core::proof_system::*;
use zokrates_field::Field;


pub fn generate_proof<
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