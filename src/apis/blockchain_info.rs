use crate::*;

use std::collections;

const API_URL: &'static str = "https://blockchain.info/ticker";

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Price {
    last: Decimal,
}

pub(crate) async fn brl_to_btc(amount: Decimal) -> error::Result<Decimal> {
    let prices: collections::HashMap<String, Price> = reqwest::get(API_URL).await?.json().await?;

    let Some(price) = prices.get("BRL") else {
        return Err(error::Error::PricesNotAvailable);
    };

    Ok(amount / price.last)
}
