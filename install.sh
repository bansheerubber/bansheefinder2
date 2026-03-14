#!/bin/sh
cargo build --release
cp ./target/release/bansheefinder3 $HOME/.local/bin/bansheefinder3
