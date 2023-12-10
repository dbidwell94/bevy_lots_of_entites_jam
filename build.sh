rm -rdf out out.zip
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rimguard_realms.wasm
cp -r assets out/assets
cp index.css out/index.css
cp index.html out/index.html
zip -r out.zip out