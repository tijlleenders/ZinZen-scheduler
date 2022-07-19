set -euxo pipefail

out=js-api

cargo build --release --target=wasm32-unknown-unknown

echo -n 'Creating out dir...'
mkdir -p $out/src;
echo ' ok'

echo -n 'Generating wasm module and JS bindings...'
wasm-bindgen \
  --target bundler \
  --out-dir $out/src \
  target/wasm32-unknown-unknown/release/scheduler.wasm;
echo ' ok'