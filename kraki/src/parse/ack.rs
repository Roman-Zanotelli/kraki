use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct Ack{
    method: Method,
    result: AckResult,
    success: bool,
    error: Option<String>,
    time_in: String,
    time_out: String,
    req_id: Option<i64> //find out what type of integer
}

#[derive(Debug, Deserialize)]
pub struct AckResult{
    channel: String,
    symbol: String,
    snapshot: Option<bool>,
    warnings: Option<Vec<String>>,
    event_trigger: Option<String>,
    depth: Option<i16>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method{
    Subscribe, Unsubscribe
}