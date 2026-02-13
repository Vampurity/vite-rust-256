#!/bin/bash

echo "🔨 Building Rust WebAssembly module..."

# 构建 WebAssembly
cargo build --target wasm32-unknown-unknown --release

# 生成 JavaScript 绑定
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/vue_rust_crypto.wasm

echo "✅ WebAssembly构建完成!"
echo "📁 Generated files in pkg/:"
ls -la pkg/
