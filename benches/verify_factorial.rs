use criterion::{criterion_group, criterion_main, Criterion};
use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::DefaultGateSerializer;
use std::fs;

pub(crate) fn benchmark_verification(c: &mut Criterion) {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let proof_bytes = fs::read("proof_with_public_inputs.bin").unwrap();
    let verifier_data_bytes = fs::read("verifier_data.bin").unwrap();

    let verifier =
        VerifierCircuitData::<F, C, D>::from_bytes(verifier_data_bytes, &DefaultGateSerializer)
            .unwrap();

    let proof =
        ProofWithPublicInputs::<F, C, D>::from_bytes(proof_bytes, &verifier.common).unwrap();

    c.bench_function("Verifier::verify", |b| {
        b.iter(|| verifier.verify(proof.clone()).unwrap());
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_verification(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
