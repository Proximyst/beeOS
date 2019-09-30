#!/usr/bin/env bash

RUST_TARGET_PATH=$(pwd) xargo build --release --target x86_64-beeos
