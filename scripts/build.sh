# Build script group_wallet canister

# Generate candid
cargo test candid -p group_wallet

# Build wasm
cargo build -p group_wallet --release --target wasm32-unknown-unknown

# Gzip wasm
gzip -c target/wasm32-unknown-unknown/release/group_wallet.wasm > target/wasm32-unknown-unknown/release/group_wallet.wasm.gz

# Copy wasm
cp target/wasm32-unknown-unknown/release/group_wallet.wasm.gz wasm/group_wallet.wasm.gz
