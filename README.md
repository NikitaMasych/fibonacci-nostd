# Plonky2 Factorial no-std | WASM

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

Launch this command to end-to-end circuit launch; output the needed data in files:

```bash
cargo run --bin factorial
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

[Check out the exact line](https://github.com/NikitaMasych/factorial-nostd/blob/main/src/bin/factorial.rs#L44)

So, even though verification part works in wasm, plonky2 in its completion may not be supported.
