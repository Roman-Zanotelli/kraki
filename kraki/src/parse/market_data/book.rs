use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Book{
    asks: Vec<AskBid>,
    bids: Vec<AskBid>,
    checksum: i64,
    symbol: String,
    timestamp: Option<String>
}
#[derive(Debug, Deserialize)]
pub struct AskBid{
    price: f64,
    qty: f64
}