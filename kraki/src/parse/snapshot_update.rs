use serde::Deserialize;

use crate::parse::market_data::{book::Book, candles::Candles, orders::Orders, ticker::Ticker, trades::Trades};


#[derive(Debug, Deserialize)]
pub struct SnapshotUpdate{
    channel: String,
    #[serde(rename = "type")]
    update_type: UpdateType,
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data{
    Trades(Vec<Trades>), Ticker(Vec<Ticker>), Candles(Vec<Candles>), Book(Vec<Book>),  Orders(Vec<Orders>)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateType{
    Snapshot, Update
}