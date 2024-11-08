use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::DefaultGateSerializer;

fn main() {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let proof_bytes = std::fs::read("proof_with_public_inputs.bin").unwrap();
    let verifier_data_bytes = std::fs::read("verifier_data.bin").unwrap();

    let verifier =
        VerifierCircuitData::<F, C, D>::from_bytes(verifier_data_bytes, &DefaultGateSerializer)
            .unwrap();

    let proof =
        ProofWithPublicInputs::<F, C, D>::from_bytes(proof_bytes, &verifier.common).unwrap();

    verifier.verify(proof).unwrap();

    println!("Verified!");
}
