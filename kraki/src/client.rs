

use tokio::sync::broadcast;

use crate::{cache::StateStream, pairs::DefaultPair, parse::{constants::KrakenChannel, market_data::{book::Book, candles::Candles, instruments::Instruments, orders::Orders, ticker::Ticker, trades::Trades}}};



pub struct Client(broadcast::Sender<()>);

impl Client{
    async fn connect(url: &str) -> Result<Self, ()>{
        todo!()
    }
    async fn book(&self, pair: Vec<DefaultPair>) -> Vec<StateStream<Book>>{
        todo!()
    }
    async fn candles(&self, pair: Vec<DefaultPair>) -> Vec<StateStream<Candles>>{
        todo!()
    }
    async fn intruments(&self, pair: Vec<DefaultPair>) -> StateStream<Instruments>{
        todo!()
    }
    async fn orders(&self, pair: Vec<DefaultPair>) -> Vec<StateStream<Orders>>{
        todo!()
    }
    async fn ticker(&self, pair: Vec<DefaultPair>) -> Vec<StateStream<Ticker>>{
        todo!()
    }
    async fn trades(&self, pair: Vec<DefaultPair>) -> Vec<StateStream<Trades>>{
        todo!()
    }
    async fn unsubscribe(&self, pair: Vec<DefaultPair>, channel: KrakenChannel){

    }
    async fn close(self){
        self.0.send(());
        self.0.closed().await;
        drop(self);
    }
}

impl Drop for Client{
    fn drop(&mut self) {
        self.0.send(());
    }
}