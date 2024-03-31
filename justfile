
# Get help
help:
	just --list

test:
	cargo run -- --port 8180

build-release:
	cargo build --release
	patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 target/release/hachha-dev

upload: build-release
	#!/usr/bin/env sh
	server_ip=$(host hachha.dev | head -1 | cut -f4 -d" ")
	ssh root@${server_ip} -f "pkill hachha-dev"
	scp ./target/release/hachha-dev root@${server_ip}:~/hachha-dev
	ssh root@${server_ip} -f "./hachha-dev &"
