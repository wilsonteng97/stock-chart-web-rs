use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, routing::get, Json,
    Router,
};
use serde_json::json;
use std::{error::Error, sync::Arc};
use time::macros::datetime;
use tokio::sync::Mutex;
use yahoo_finance_api::{Quote, YahooConnector, YahooError};

type AppState = Arc<Mutex<YahooConnector>>;

async fn get_quotes(yahoo: &YahooConnector) -> Result<Vec<Quote>, YahooError> {
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    let resp = yahoo.get_quote_history("AAPL", start, end).await?;
    resp.quotes()
}

#[debug_handler]
async fn landing_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let yahoo = state.lock().await;
    match get_quotes(&yahoo).await {
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
    let yahoo = YahooConnector::new();
    let yahoo = Arc::new(Mutex::new(yahoo));
    let app = Router::new()
        .route("/", get(landing_page_handler))
        .with_state(yahoo);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
