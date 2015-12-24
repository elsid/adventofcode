all: 1_1 1_2 2_1 3_1 4_1 5_1 5_2 6_1

1_1: rust_1_1
1_2: rust_1_2
2_1: python_2_1
3_1: python_3_1
4_1: python_4_1
5_1: rust_5_1
5_2: rust_5_2
6_1: rust_6_1

python_1_1: src/1_1.py var/1.txt
	cat var/1.txt | python3 src/1_1.py

python_2_1: src/2_1.py var/2.txt
	cat var/2.txt | python3 src/2_1.py

python_3_1: src/3_1.py var/3.txt
	cat var/3.txt | python3 src/3_1.py

python_4_1: src/4_1.py var/4.txt
	cat var/4.txt | python3 src/4_1.py

rust_1_1: target/release/1_1 var/1.txt
	cat var/1.txt | target/release/1_1

rust_1_2: target/release/1_2 var/1.txt
	cat var/1.txt | target/release/1_2

rust_5_1: target/release/5_1 var/5.txt
	cat var/5.txt | target/release/5_1

rust_5_2: target/release/5_2 var/5.txt
	cat var/5.txt | target/release/5_2

rust_6_1: target/release/6_1 var/6.txt
	cat var/6.txt | target/release/6_1

$(patsubst src/%.rs, target/release/%, $(wildcard src/*.rs)): $(wildcard src/%.rs)
	@echo $(wildcard src/*.rs)
	cargo build --release

.PHONY: all 1_1 1_2 2_1 3_1 4_1 5_1 5_2 6_1 \
	python_1_1 python_2_1 python_3_1 python_4_1 \
	rust_1_1 rust_1_2 rust_5_1 rust_5_2 rust_6_1
