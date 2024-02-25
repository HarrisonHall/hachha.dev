# hachha.dev
2023 attempt at a new personal website.
Site compiles into a single executable w/ embedded content.
View it at [hachha.dev](http://hachha.dev)!

## Building & Serving
### From Nixos
```fish
cargo build --release
patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 target/release/hachha-dev
scp target/release/hachha-dev root@$<SERVER>:~/hachha-dev
SSH root@$<SERVER>
./hachha-dev [--debug]
```
### Using Zig
- Install `zig` and run `cargo install zigbuild`
- `cargo zigbuild --target aarch64-unknown-linux-gnu`

## Stack
Site is served with an async runtime powered by tokio-axum.
Templating is powered by handlebars.
Styling is thanks to spectre.css.

## Style & Design
- Site should only panic before launch
  - `unwrap()`s are only used during `Site` creation, all `Results` during
    serving are checked