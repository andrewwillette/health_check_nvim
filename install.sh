#!/bin/sh
cargo build
cp target/debug/libhealth_check_nvim.dylib lua/health_check_nvim.so
