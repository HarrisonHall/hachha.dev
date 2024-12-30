# Actions for hachha.dev

# List directives
[private]
default:
	@just --list -u

# Test locally
test:
	(sleep 1 && firefox 127.0.0.1:8180) &
	cargo run -- --port 8180 --debug

# Build release
build-release:
	cargo build --release
	patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 target/release/hachha-dev

# SSH to server
ssh:
	ssh root@50.116.32.107

# Upload to server
upload: build-release
	#!/usr/bin/env sh
	server_ip=$(host hachha.dev | head -1 | cut -f4 -d" ")
	ssh root@${server_ip} -f "pkill hachha-dev"
	scp ./target/release/hachha-dev root@${server_ip}:~/hachha-dev
	ssh root@${server_ip} -f "./hachha-dev &"
