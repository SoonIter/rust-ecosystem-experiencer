#!/bin/sh

echo "start building"
cargo build --release --target-dir ./target

echo "copying output"
mkdir output
cp scripts/bootstrap.sh output && chmod +x output/bootstrap.sh
mv target/release/warp_demo output
cp -r ./web output/web
echo "done"