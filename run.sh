#!/usr/bin/env bash
cargo build --release

printf '1.1: '
cat var/1.txt | target/release/1_1

printf '1.2: '
cat var/1.txt | target/release/1_2

printf '5.1: '
cat var/5.txt | target/release/5_1
