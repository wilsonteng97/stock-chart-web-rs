# Simple Interactive Stock Chart 📈

Simple interactive stock chart using:
- [`axum`](https://github.com/tokio-rs/axum) to serve the backend and the front end
- [`yahoo_finance_api`](https://crates.io/crates/yahoo_finance_api) to query the Yahoo Finance API
- [`rust_embed`](https://crates.io/crates/rust-embed) to embed the frontend in the release binary.
- [`tower-livereload`](https://crates.io/crates/tower-livereload) for convenient non-release devemopment.
- [`chart.js`](https://www.chartjs.org/) for the front end (plain vanilla JS)

For now, it's using a [fork](https://github.com/meuter/yahoo_finance_api) of 
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

Visit the URL [`http://127.0.0.1:3000?ticker=AAPL`](http://127.0.0.1?ticker=AAPL). You can, of course
change the ticker with anything you'd like.

If any file in the `assets` folder is changed, the frontend automagically be reloaded.

# Release Build

In order to build the app in release mode:

```bash
cargo build -r
```

The resulting binary will be completely self contained, as in: it embeds the entire `assets` folder.

# Screenshot

![screenshot](/screenshot.png?raw=true)
