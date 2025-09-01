


use futures_util::{SinkExt, StreamExt};
use tokio::{sync::{broadcast, mpsc::{self}}, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::Message};


pub struct Connection{
    sender: tokio::sync::mpsc::Sender<Message>,
    reader: tokio::sync::broadcast::Sender<Message>,
    sender_handle: JoinHandle<()>,
    reader_handle: JoinHandle<()>,
}
impl Connection{
    pub async fn connect(url: &str) -> Result<Self, ()>{
        let (sender, mut recver) = mpsc::channel::<Message>(32);
        let (broadcaster, _) = broadcast::channel::<Message>(usize::MAX);
        let (mut ws_writer, ws_reader) = connect_async(url).await
        .map_err(|err| todo!())?.0
        .split();
    
        Ok(Self{
            sender,
            reader: broadcaster.clone(),
            sender_handle: tokio::spawn(async move{
                while let Some(msg) = recver.recv().await {
                    match ws_writer.send(msg).await{
                        Ok(_) => {},
                        Err(_err) => {
                            todo!()
                        },
                    };
                }
            }),
            reader_handle: tokio::spawn(async move{
                ws_reader.for_each(|msg| async{
                    let _ = match msg{
                        Ok(msg) => match broadcaster.send(msg){
                            Ok(subs) => {
                                todo!()
                            },
                            Err(err) => {
                                todo!()
                            },
                        },
                        Err(_err) => todo!(),
                    };
                }).await;
            }),
        })
    }
    pub async fn send(&self, msg: Message) -> Result<(), ()>{
        self.sender.send(msg).await.map_err(|err|todo!())
    }
    pub fn read(&self) -> tokio::sync::broadcast::Receiver<Message>{
        self.reader.subscribe()
    }
    pub fn close(mut self) -> Result<(), ()>{
        todo!()
    }
}