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
cd ../.. || exit
