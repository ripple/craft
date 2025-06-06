cd xrpl-std || exit
cargo build
cargo build --target wasm32-unknown-unknown
cd ..
cd ./wasm-host || exit
cargo build
cd ../projects/xrpl_std_example || exit
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
<<<<<<< HEAD
cd ../.. || exit
=======
cd ../.. || exit
>>>>>>> origin/main
