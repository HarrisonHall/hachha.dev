# hachha.dev
2023 attempt at a new personal website.
Site compiles into a single executable.

## Serving
- `./hachha-dev [--port PORT] [--debug]`

## Building
- `cargo build --release`

## Stack
Site is served with an async runtime powered by tokio-axum.
Templating is powered by handlebars.