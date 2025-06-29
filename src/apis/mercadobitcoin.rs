use crate::*;

const API_URL: &'static str = "https://www.mercadobitcoin.net/api/BTC/ticker";

#[derive(Debug, serde::Deserialize)]
struct Ticker {
    last: Decimal,
}

#[derive(Debug, serde::Deserialize)]
struct Request {
    ticker: Ticker,
}

pub(crate) async fn brl_to_btc(amount: Decimal) -> error::Result<Decimal> {
    let price: Request = reqwest::get(API_URL).await?.json().await?;

    Ok(amount / price.ticker.last)
}
