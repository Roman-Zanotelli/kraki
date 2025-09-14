use std::sync::Arc;

use tokio::sync::broadcast::Sender;

use tokio_stream::wrappers::WatchStream;

use crate::{pairs::DefaultPair, parse::{constants::KrakenChannel, market_data::{book::Book, candles::Candles, instruments::Instruments, orders::Orders, ticker::Ticker, trades::Trades}}};

type UpdateStream<T> = WatchStream<Arc<T>>;

pub struct Client(Sender<()>, );

impl Client{
    async fn connect(url: &str) -> Result<Self, ()>{
        todo!()
    }
    async fn book(&self, pair: Vec<DefaultPair>) -> Vec<UpdateStream<Book>>{
        todo!()
    }
    async fn candles(&self, pair: Vec<DefaultPair>) -> Vec<UpdateStream<Candles>>{
        todo!()
    }
    async fn intruments(&self, pair: Vec<DefaultPair>) -> UpdateStream<Instruments>{
        todo!()
    }
    async fn orders(&self, pair: Vec<DefaultPair>) -> Vec<UpdateStream<Orders>>{
        todo!()
    }
    async fn ticker(&self, pair: Vec<DefaultPair>) -> Vec<UpdateStream<Ticker>>{
        todo!()
    }
    async fn trades(&self, pair: Vec<DefaultPair>) -> Vec<UpdateStream<Trades>>{
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