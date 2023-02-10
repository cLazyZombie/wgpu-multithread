set -ex
# cargo build --workspace --release --target wasm32-unknown-unknown
RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals --cfg=web_sys_unstable_apis" cargo build --target=wasm32-unknown-unknown --release -Z build-std=panic_abort,std
wasm-bindgen --target no-modules --out-dir ./web ./target/wasm32-unknown-unknown/release/wgpu_multithread.wasm
