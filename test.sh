for dir in ./projects/*/; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "🔧 Building in $dir"
        (cd "$dir" && cargo rustc --target wasm32-unknown-unknown -- -D warnings && cargo build --target wasm32-unknown-unknown) || exit 1
    fi
done
