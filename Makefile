all: 1_1 1_2 2_1 3_1 4_1 5_1 5_2 6_1 6_2 7_1 7_2 10_1 10_2 14_1

clean:
	rm -rf target/release/*

1_1: rust_1_1
1_2: rust_1_2
2_1: python_2_1
3_1: python_3_1
4_1: python_4_1
5_1: rust_5_1
5_2: rust_5_2
6_1: rust_6_1
6_2: rust_6_2
7_1: rust_7_1
7_2: rust_7_2
10_1: rust_10_1
10_2: rust_10_2
14_1: rust_14_1

python_1_1: src/1_1.py var/1.txt
	cat var/1.txt | python3 src/1_1.py

python_2_1: src/2_1.py var/2.txt
	cat var/2.txt | python3 src/2_1.py

python_3_1: src/3_1.py var/3.txt
	cat var/3.txt | python3 src/3_1.py

python_4_1: src/4_1.py var/4.txt
	cat var/4.txt | python3 src/4_1.py

python_7_1: src/7_1.py var/7.txt
	cat var/7.txt | python3 src/7_1.py

python_7_2: src/7_2.py var/7.txt
	cat var/7.txt | python3 src/7_2.py

rust_1_1: target/release/1_1 var/1.txt
	cat var/1.txt | target/release/1_1

rust_1_2: target/release/1_2 var/1.txt src/1_2.rs
	cat var/1.txt | target/release/1_2

rust_5_1: target/release/5_1 var/5.txt src/5_1.rs
	cat var/5.txt | target/release/5_1

rust_5_2: target/release/5_2 var/5.txt src/5_2.rs
	cat var/5.txt | target/release/5_2

rust_6_1: target/release/6_1 var/6.txt src/6_1.rs
	cat var/6.txt | target/release/6_1

rust_6_2: target/release/6_2 var/6.txt src/6_2.rs
	cat var/6.txt | target/release/6_2

rust_7_1: target/release/7_1 var/7.txt src/7_1.rs
	cat var/7.txt | target/release/7_1

rust_7_2: target/release/7_2 var/7.txt src/7_2.rs
	cat var/7.txt | target/release/7_2

rust_10_1: target/release/10_1 var/10.txt src/10_1.rs
	cat var/10.txt | target/release/10_1

rust_10_2: target/release/10_2 var/10.txt src/10_2.rs
	cat var/10.txt | target/release/10_2

rust_14_1: target/release/14_1 var/14.txt src/14_1.rs
	cat var/14.txt | target/release/14_1

$(patsubst src/%.rs, target/release/%, $(wildcard src/*.rs)): $(wildcard src/%.rs)
	cargo build --release

.PHONY: all 1_1 1_2 2_1 3_1 4_1 5_1 5_2 6_1 6_2 7_1 7_2 10_1 10_2 14_1 \
	python_1_1 python_2_1 python_3_1 python_4_1 python_7_1 python_7_2 \
	rust_1_1 rust_1_2 rust_5_1 rust_5_2 rust_6_1 rust_6_2 rust_7_1 rust_7_2 rust_10_1 rust_10_2 \
	rust_14_1
