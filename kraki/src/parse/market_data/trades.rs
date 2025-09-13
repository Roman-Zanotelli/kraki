use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trades{
    symbol: String,
    side: String,
    qty: f64,
    price: f64,
    ord_type: String,
    trade_id: i64,
    timestamp: String
}