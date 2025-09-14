use serde::{Deserialize, Serialize};

use crate::parse::constants::{KrakenChannel, KrakenMethod};

#[derive(Debug, Deserialize)]
pub(crate) struct Ack{
    method: KrakenMethod,
    result: AckResult,
    success: bool,
    error: Option<String>,
    time_in: String,
    time_out: String,
    req_id: Option<i64> //find out what type of integer
}

#[derive(Debug, Deserialize)]
struct AckResult{
    channel: KrakenChannel,
    symbol: String,
    snapshot: Option<bool>,
    warnings: Option<Vec<String>>,
    event_trigger: Option<String>,
    depth: Option<i16>
}
