cd xrpl-std || exit
cargo build
cargo build --target wasm32-unknown-unknown
cd ..
cd ./wasm-host || exit
cargo build
cd .. || exit
for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo build --target wasm32-unknown-unknown && cargo build --target wasm32-unknown-unknown --release) || exit 1
  fi
done

echo "✅  All WASM builds completed successfully"

for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo build && cargo build --release) || exit 1
  fi
done

echo "✅  All Rust builds completed successfully"

for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo fmt --all -- --check) || exit 1
  fi
done

echo "✅  All 'cargo fmt' checks completed successfully"

for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo clippy --all-targets --all-features) || exit 1
  fi
done

echo "✅  All 'cargo clippy' checks completed successfully"

cd ../.. || exit
