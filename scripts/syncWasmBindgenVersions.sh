WASM_BINDGEN_VERSION=$(wasm-bindgen -V | cut -d " " -f 2)

echo 'Syncing wasm-bindgen version in crate with that of the installed CLI...'
# hack, will not work if we have multiple "version = ..." in Cargo.toml
sed -i "s/^version\ =.*$/version = \"=$WASM_BINDGEN_VERSION\"/" Cargo.toml
cargo update --package wasm-bindgen