clear
cd ./core
cargo build --target wasm32-unknown-unknown
wasm-bindgen \
    --target web \
    --out-dir "../gui/src/lib/assets/wasm" \
    --out-name "bevy-defense" \
    ./target/wasm32-unknown-unknown/debug/bevy-defense.wasm
cd ..

cp -r ./assets/ ./gui/static/

# cp gui/src/index.html build/dev