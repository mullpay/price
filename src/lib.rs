mod apis;
mod error;

pub use error::*;
use rust_decimal::Decimal;

pub async fn brl_to_btc(amount: Decimal) -> error::Result<Decimal> {
    let mut prices = Vec::new();

    match apis::blockchain_info::brl_to_btc(amount).await {
        Ok(p) => prices.push(p),
        Err(e) => log::error!("blockchain_info: {e:?}"),
    };

    match apis::mercadobitcoin::brl_to_btc(amount).await {
        Ok(p) => prices.push(p),
        Err(e) => log::error!("mercadobitcoin: {e:?}"),
    };

    let Some(price) = prices.get(prices.len() / 2) else {
        return Err(error::Error::PricesNotAvailable);
    };

    Ok(*price)
}
