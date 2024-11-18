# Plonky2 Fibonacci no-std | WASM

This repository demonstrates that plonky2 proof verification works in wasm.

## Execute

```bash
wasm-pack build --target web
```

```bash
npx http-server . -o
```

## Data 

You can see how `proof_with_public_inputs.bin` and `verifier_data.bin` are composed in `src/bin`

Launch this command to trigger circuit end-to-end; output the needed data in files:

```bash
cargo run --bin fibonacci
```

Launch this command to see how verification only happens using deserialized info.

```bash
cargo run --bin verify
```

## Additional info

After trying to run end-to-end circuit execution, including building and proving, in wasm,
it was discovered that failure happens on this step:

```rust
let data = builder.build::<C>();
```

[Check out the exact line](https://github.com/NikitaMasych/fibonacci-nostd/blob/main/src/bin/fibonacci.rs#L44)

So, even though verification part works in wasm, plonky2 in its completion may not be supported.

## Benches

You can compare performance of the factorial circuit verification in WASM and native rust.

Environment: `MacBook Pro M2 Max 32 GB 1 TB Storage`

Rust:

```bash
cargo bench
```

WASM:

Same steps for executing, you will see timings in browser console.

### Results

Rust: 1.13 ms
WASM: 13 ms