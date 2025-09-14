use serde::Deserialize;

use crate::parse::{constants::{KrakenChannel, KrakenUpdateType}, market_data::{book::Book, candles::Candles, instruments::Instruments, orders::Orders, ticker::Ticker, trades::Trades}};


#[derive(Debug, Deserialize)]
pub struct SnapshotUpdate{
    channel: KrakenChannel,
    #[serde(rename = "type")]
    update_type: KrakenUpdateType,
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data{
    Trades(Vec<Trades>), Ticker(Vec<Ticker>), Candles(Vec<Candles>), Book(Vec<Book>),  Orders(Vec<Orders>), Instruments(Instruments)
}