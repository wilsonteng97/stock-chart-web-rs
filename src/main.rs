use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

#[cfg(not(debug_assertions))]
use axum_embed::ServeEmbed;

use notify::Watcher;
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;

use serde::Deserialize;
use serde_json::json;
use std::{error::Error, path::Path, sync::Arc};
use tokio::sync::Mutex;
#[cfg(debug_assertions)]
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_livereload::LiveReloadLayer;
use yahoo_finance_api::{Quote, YahooConnector, YahooError};

type AppState = Arc<Mutex<YahooConnector>>;

#[derive(Deserialize, Debug, Clone)]
struct QuoteApiQueryParameters {
    pub ticker: String,
    pub interval: Option<String>,
    pub period: Option<String>,
}

#[cfg(not(debug_assertions))]
#[derive(RustEmbed, Clone)]
#[folder = "assets"]
struct Assets;

async fn get_quotes(
    yahoo: &YahooConnector,
    ticker: impl AsRef<str>,
    interval: impl AsRef<str>,
    period: impl AsRef<str>,
) -> Result<Vec<Quote>, YahooError> {
    let resp = yahoo
        .get_quote_range(ticker.as_ref(), interval.as_ref(), period.as_ref())
        .await?;
    resp.quotes()
}

#[debug_handler]
async fn landing_page_handler(
    State(state): State<AppState>,
    Query(QuoteApiQueryParameters {
        ticker,
        interval,
        period,
    }): Query<QuoteApiQueryParameters>,
) -> impl IntoResponse {
    let yahoo = state.lock().await;
    let interval = interval.unwrap_or("1d".into());
    let period = period.unwrap_or("1y".into());

    match get_quotes(&yahoo, ticker, interval, period).await {
        Ok(quotes) => {
            let status = StatusCode::OK;
            let payload = Json(quotes);
            (status, payload).into_response()
        }
        Err(error) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let payload = Json(json!({ "error": error.to_string() }));
            (status, payload).into_response()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let debug_level = std::env::var("TRACE_LEVEL")
        .unwrap_or("INFO".into())
        .parse::<tracing::Level>()?;

    tracing_subscriber::fmt().with_max_level(debug_level).init();

    #[cfg(debug_assertions)]
    let assets = ServeDir::new("assets");

    #[cfg(not(debug_assertions))]
    let assets = ServeEmbed::<Assets>::new();

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let yahoo = YahooConnector::new();
    let yahoo = Arc::new(Mutex::new(yahoo));
    let app = Router::new()
        .route("/api/v1/quotes", get(landing_page_handler))
        .nest_service("/", assets)
        .layer(LiveReloadLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(yahoo);

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(Path::new("assets"), notify::RecursiveMode::Recursive)?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
