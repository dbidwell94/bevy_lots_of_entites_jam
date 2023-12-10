cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/lots_of_entities.wasm
cp -r assets out/assets
cp index.css out/index.css
cp index.html out/index.html
zip -r out.zip out