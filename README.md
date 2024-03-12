# Simple Interactive Stock Chart

Simple interactive stock chart.
- backend: Rust with [`axum`](https://github.com/tokio-rs/axum) using
  [`yahoo_finance_api`](https://crates.io/crates/yahoo_finance_api)
- frontend: Vanilla JS with [`chart.js`](https://www.chartjs.org/)

For now, using a [fork](https://github.com/meuter/yahoo_finance_api) of 
`yahoo_finance_api` because `Quote` are not serializable. Once this 
[issue](https://github.com/xemwebe/yahoo_finance_api/issues/40) is closed,
this fork will not be necessary anymore.

# Development

In order to work on the frontend / backend, the simplest way is to install `cargo-watch`:

```bash
cargo install cargo-watch
```

and execute `cargo run`  on code change:

```bash
cargo watch -x run 
```

# Release Build

In order to build the app in release mode:

```bash
cargo build -r
```

The resulting binary embeds the `assets` folder thanks to [`rust_embed`](https://crates.io/crates/rust-embed)
and [`axum_embed`](https://crates.io/crates/axum-embed).

# Screenshot

![screenshot](/screenshot.png?raw=true)
