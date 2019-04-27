#!/usr/bin/env bash

VER=$(cat Cargo.toml |sed -n 's/^version = "\(.*\)"/\1/p')
echo "pub const VERSION: &str = \"${VER}\";" > src/constants.rs
