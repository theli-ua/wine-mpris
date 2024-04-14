#/bin/bash
set -e
cargo build --release --target i686-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
