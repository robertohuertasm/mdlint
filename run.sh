#!/bin/bash

cargo clippy
cargo fmt
cargo run -p rusty-markdownlint-cli