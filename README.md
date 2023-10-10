# DMA Web static

Main pupose of this repository is to test wasm compilation from rust code. Made from template https://github.com/rustwasm/wasm_game_of_life and modified according to https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm and bevy game engine examples (removing the npm package, it's just plain javascript and HTML now).

build with:

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name dma_web --out-dir pkg --target web target/wasm32-unknown-unknown/release/dma_web.wasm
```

