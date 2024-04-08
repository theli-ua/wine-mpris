#/bin/bash
set -e
cargo build --release --target i686-unknown-linux-gnu
winegcc -shared -spec windows.media.mediacontrol.spec -Wl,--as-needed target/i686-unknown-linux-gnu/release/libmediacontrol.a -loleaut32 -lpropsys -lcombase -m32 -o windows.media.mediacontrol.so
