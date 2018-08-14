#!/bin/bash

cargo clippy --all -- -D clippy_pedantic -A non-ascii-literal
cargo fmt
cargo run -p rusty-markdownlint-cli