use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum KrakenUpdateType{
    Snapshot, Update
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum KrakenChannel{
    Ticker, Book, Level3, Ohlc, Trade, Instrument
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum KrakenMethod{
    Subscribe, Unsubscribe
}