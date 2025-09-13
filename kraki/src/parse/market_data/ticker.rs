use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ticker{
    ask: f64,
    ask_qty: f64,
    bid: f64,
    bid_qty: f64,
    change: f64,
    change_pct: f64,
    high: f64,
    last: f64,
    low: f64,
    symbol: String,
    volume: f64,
    vwap: f64, 
}