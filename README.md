# Plonky2 Factorial no-std | WASM

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