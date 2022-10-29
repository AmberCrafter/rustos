#!/bin/bash

# install rust tool
cargo install cargo-xbuild
rustup component add rust-src
rustup component add llvm-tools-preview

# install qemu
sudo apt-get update
audo apt-get install -y qemu-system-x86 gdb