# Build script multisig canister

# Generate candid
cargo test candid -p multisig

# Build wasm
cargo build -p multisig --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/multisig.wasm > target/wasm32-unknown-unknown/release/multisig.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/multisig.wasm.gz wasm/multisig.wasm.gz
