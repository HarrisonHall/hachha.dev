# hachha.dev
2023 attempt at a new personal website.
Site compiles into a single executable.
View it at [hachha.dev](http://hachha.dev)!

> **Warning**
> This site is still under construction

## Building & Serving
```fish
cargo build --release
patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 target/release/hachha-dev
set IP "CUSTOM_IP"
scp target/release/hachha-dev root@$IP:~/hachha-dev
SSH $IP
./hachha-dev [--debug]
```

## Stack
Site is served with an async runtime powered by tokio-axum.
Templating is powered by handlebars.
Styling is thanks to spectre.css.

## Style & Design
- Site should only panic before launch
  - `unwrap()`s are only used during `Site` creation, all `Results` during
    serving are checked

## TODO
- [ ] Styling
- [ ] Blog styling
- [ ] Resume
- [ ] Projects page
  - [ ] Projects modal
- [ ] Landing page information
- [ ] Games page (link to [trackl.space](trackl.space)?)