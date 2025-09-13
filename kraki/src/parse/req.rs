use serde::Deserialize;

use crate::parse::ack::Method;


#[derive(Debug, Deserialize)]
pub struct Req{
    method: Method,
    params: ReqParam,
    req_id: i64
}


#[derive(Debug, Deserialize)]
pub struct ReqParam{
    channel: String,
    symbol: Option<Vec<String>>,
    event_trigger: Option<String>,
    snapshot: Option<bool>,
    depth: Option<i64>,
    token: Option<String>,
    interval: Option<i64>,
    include_tokenized_assets: Option<bool>,
}