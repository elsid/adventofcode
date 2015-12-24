#!/usr/bin/env bash
cargo build --release

printf 'rust 1.1: '
cat var/1.txt | target/release/1_1

printf 'rust 1.2: '
cat var/1.txt | target/release/1_2

printf 'python 1.1: '
cat var/1.txt | python3 src/1_1.py

printf 'python 2.1: '
cat var/2.txt | python3 src/2_1.py

printf 'python 3.1: '
cat var/3.txt | python3 src/3_1.py

printf 'python 4.1: '
cat var/4.txt | python3 src/4_1.py

printf 'rust 5.1: '
cat var/5.txt | target/release/5_1
