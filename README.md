# angela
merkle tree construction and leaf verification with v.s. without patched sha256 crate.

Steps to reproduce:

```
export SP1_PROVER=network
export NETWORK_PRIVATE_KEY=<YOUR_PRIVATE_KEY>
export NETWORK_RPC_URL=https://rpc.production.succinct.xyz

cd program && cargo prove build

cd ../script

RUST_LOG=info cargo run --release -- --execute

RUST_LOG=info cargo run --release -- --prove
```

