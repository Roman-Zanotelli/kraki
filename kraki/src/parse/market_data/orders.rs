use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Orders{
    symbol: String,
    timestamp: Option<String>,
    checksum: Option<i64>,
    bids: Vec<AskBid>,
    asks: Vec<AskBid>
}

#[derive(Debug, Deserialize)]
pub struct AskBid{
    event: Option<Event>,
    order_id: String,
    limit_price: f64,
    order_qty: f64,
    timestamp: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Add, Modify, Delete
}