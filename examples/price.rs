use rust_decimal::dec;

#[tokio::main]
async fn main() {
    let price = price::brl_to_btc(dec!(581900)).await.unwrap();
    dbg!(price);
}
