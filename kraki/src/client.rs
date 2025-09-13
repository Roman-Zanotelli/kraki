use std::sync::Arc;

use futures_util::StreamExt;
use tokio::{spawn, sync::mpsc::{self, channel}, task::JoinHandle};
use tokio_tungstenite::connect_async;

use tokio_stream::wrappers::WatchStream;


pub enum KrakiCommand {

}

pub struct Client(mpsc::Sender<KrakiCommand>, JoinHandle<()>);

impl Client{
    async fn connect(url: &str) -> Result<Self, ()>{
        
        //Establish connection with sink & stream halves
        let (mut ws_sink, ws_stream) = connect_async(url).await.map_err(|_err|{
                todo!("Remap errs")
            })?.0
            .split();
        
        let (cmd, mut _cmd) = channel::<KrakiCommand>(100);
       
        todo!()
    }
}
