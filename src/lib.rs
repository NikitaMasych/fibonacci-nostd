#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use anyhow::Result;
use plonky2::plonk::circuit_data::VerifierCircuitData;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::util::serialization::DefaultGateSerializer;
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

use alloc::vec::Vec;

#[wasm_bindgen]
pub fn verify_proof(proof_bytes: &[u8], verifier_data_bytes: &[u8]) -> Result<(), JsValue> {
    console::log_1(&"Starting proof verification".into());

    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    console::log_1(&"Deserializing verifier data".into());
    let verifier = VerifierCircuitData::<F, C, D>::from_bytes(
        Vec::from(verifier_data_bytes),
        &DefaultGateSerializer,
    )
    .map_err(|_| JsValue::from_str("Failed to deserialize verifier data"))?;

    console::log_1(&"Deserializing proof data".into());
    let proof =
        ProofWithPublicInputs::<F, C, D>::from_bytes(Vec::from(proof_bytes), &verifier.common)
            .map_err(|_| JsValue::from_str("Failed to deserialize proof data"))?;

    console::log_1(&"Verifying proof".into());

    // Get the Performance object to measure time
    let performance = window()
        .ok_or_else(|| JsValue::from_str("Could not access window object"))?
        .performance()
        .ok_or_else(|| JsValue::from_str("Could not access performance object"))?;

    // Record start time
    let start_time = performance.now();
    
    verifier
        .verify(proof)
        .map_err(|_| JsValue::from_str("Verification failed"))?;

    // Record end time
    let end_time = performance.now();

    // Log the elapsed time
    let elapsed_time = end_time - start_time;
    console::log_2(
        &"Proof verified successfully in (ms):".into(),
        &elapsed_time.into(),
    );

    Ok(())
}
