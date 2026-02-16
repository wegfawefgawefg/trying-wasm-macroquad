#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

rustup target add wasm32-unknown-unknown >/dev/null
cargo build --release --target wasm32-unknown-unknown -p app

mkdir -p dist
cp web/index.html dist/index.html
cp web/mq_js_bundle.js dist/mq_js_bundle.js
cp target/wasm32-unknown-unknown/release/app.wasm dist/app.wasm

echo "Built web artifacts in dist/"
