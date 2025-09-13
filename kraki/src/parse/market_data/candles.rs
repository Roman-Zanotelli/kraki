use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Candles{
    symbol: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    vwap: f64,
    trades: f64,
    volume: f64,
    interval_begin: String,
    interval: i64,
    timestamp: Option<String>
}