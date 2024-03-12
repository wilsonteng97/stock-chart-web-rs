use std::error::Error;
use time::macros::datetime;
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new();
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    let resp = provider.get_quote_history("AAPL", start, end).await?;
    let quotes = resp.quotes()?;
    println!("Apple's quotes in January 2020: {:?}", quotes);

    Ok(())
}
