#![cfg_attr(target_arch = "wasm32", no_std)]

use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(not(target_arch = "wasm32"))]
use std::println;

#[wasm_bindgen]
pub fn run_factorial() -> Result<(), JsValue> {
    console::log_1(&"Starting factorial calculation".into());

    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    console::log_1(&"Config created".into());

    let mut builder = CircuitBuilder::<F, D>::new(config);
    console::log_1(&"CircuitBuilder created".into());

    // The arithmetic circuit.
    let initial = builder.add_virtual_target();
    let mut cur_target = initial;
    console::log_1(&"Initial target set".into());

    for i in 2..101 {
        let i_target = builder.constant(F::from_canonical_u32(i));
        cur_target = builder.mul(cur_target, i_target);
    }
    console::log_1(&"Arithmetic circuit completed".into());

    // Public inputs are the initial value and the result.
    builder.register_public_input(initial);
    builder.register_public_input(cur_target);
    console::log_1(&"Public inputs registered".into());

    let mut pw = PartialWitness::new();
    pw.set_target(initial, F::ONE);
    console::log_1(&"Partial witness created".into());

    let data = builder.build::<C>();
    console::log_1(&"Circuit built".into());

    let proof = data.prove(pw).map_err(|_| JsValue::from_str("Proof generation failed"))?;
    console::log_1(&"Proof generated".into());

    data.verify(proof).map_err(|_| JsValue::from_str("Verification failed"))
}
