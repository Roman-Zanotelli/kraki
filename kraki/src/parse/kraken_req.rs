use serde::{Deserialize, Serialize};

use crate::parse::constants::{KrakenChannel, KrakenMethod};



#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct KrakenReq{
    method: KrakenMethod,
    params: ReqParam,
    req_id: i64
}


#[derive(Debug, Deserialize, Serialize)]
struct ReqParam{
    channel: KrakenChannel,
    symbol: Option<Vec<String>>,
    event_trigger: Option<String>,
    snapshot: Option<bool>,
    depth: Option<i64>,
    token: Option<String>,
    interval: Option<i64>,
    include_tokenized_assets: Option<bool>,
}

impl KrakenReq{
    pub(crate) fn book(method: &KrakenMethod, symbol: &Vec<String>, depth: &i64) -> Self{
        todo!()
    }
    pub(crate) fn candles( method: &KrakenMethod, symbol: &Vec<String>, interval: &i64) -> Self{
        todo!()
    }
    pub(crate) fn instruments( method: &KrakenMethod, include_tokenized_assets: &bool) -> Self{
        todo!()
    }
    pub(crate) fn orders(method: &KrakenMethod, symbol: &Vec<String>, depth: &i64) -> Self{
        todo!()
    }
    pub(crate) fn ticker(method: &KrakenMethod, symbol: &Vec<String>, event_trigger: &String) -> Self{
        todo!()
    }
    pub(crate) fn trades(method: &KrakenMethod, symbol: &Vec<String>) -> Self{
        todo!()
    }
}