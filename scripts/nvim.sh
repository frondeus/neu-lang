#!/usr/bin/env bash

WHAT=${1:-"example.md"}

cargo build --all --all-features --release

nvim "$WHAT"
