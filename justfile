# Actions for hachha.dev.

# List directives.
[private]
default:
	@just --list -u

# Set up dependencies.
setup:
    rustup default stable
    rustup component add rust-std-x86_64-unknown-linux-musl

# Test locally
test:
	#/usr/bin/env sh
	cargo build
	sh -c "sleep 2 && xdg-open http://127.0.0.1:8180" 2&>/dev/null &
	cargo run -- --port 8180 --debug

# Build static release for many versions of linux via musl.
build-release:
    # Req: rustup component add rust-std-x86_64-unknown-linux-musl
    cargo build --target x86_64-unknown-linux-musl --release
    # patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 target/release/hachha-dev

# SSH to server.
ssh:
	#!/usr/bin/env sh
	server_ip=$(host hachha.dev | head -1 | cut -f4 -d" ")
	ssh root@{server_ip}

# Upload to server.
upload: build-release
	#!/usr/bin/env sh
	dir="workspace/dev/hachha-dev"
	server_ip=$(host hachha.dev | head -1 | cut -f4 -d" ")
	ssh root@${server_ip} -f "pkill -f hachha-dev && mkdir -p ${dir}"
	scp ./target/x86_64-unknown-linux-musl/release/hachha-dev root@${server_ip}:~/${dir}/hachha-dev
	# ssh root@${server_ip} -f "./${dir}/hachha-dev &"
