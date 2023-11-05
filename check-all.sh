#!/bin/bash

clear
cargo fmt --check --all
cargo check --workspace
cargo clippy
cargo test --workspace