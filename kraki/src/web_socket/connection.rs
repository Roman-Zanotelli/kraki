use futures_util::{SinkExt, StreamExt};
use tokio::{sync::{broadcast, mpsc::{self}}, task::JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::Message};


pub struct Connection{
    sender: tokio::sync::mpsc::Sender<Message>,
    reader: tokio::sync::broadcast::Sender<Message>,
    _sender_handle: JoinHandle<()>,
    _reader_handle: JoinHandle<()>,
    _exit: tokio::sync::broadcast::Sender<()>
}
impl Connection{


    pub async fn connect(url: &str) -> Result<Self, ()>{
        let (sender, mut recver) = mpsc::channel::<Message>(32);
        let (broadcaster, _) = broadcast::channel::<Message>(usize::MAX);
        let (_exit, _) = broadcast::channel::<()>(1);
        let exit = _exit.clone();
        let (mut ws_writer, ws_reader) = connect_async(url).await
        .map_err(|_err| {
            todo!()
        })?.0
        .split();
    
        Ok(Self{
            //Send messages to Kraken API
            sender,
            //Recieve messages from Kraken API 
            reader: broadcaster.clone(),
            //Command broadcaster
            _exit: _exit.clone(),

            //Thread to Fan-In Messages
            _sender_handle: tokio::spawn(async move{
                let exit = _exit;
                while let Some(msg) = recver.recv().await {
                    match ws_writer.send(msg).await{
                        Ok(_) => {
                            //TODO: Add Logging
                        },
                        Err(_err) => {
                            //TODO: Add logging
                            break;
                        },
                    };
                }
            }),

            //Thread to Fan-Out Messages
            _reader_handle: tokio::spawn(async move{

                let exit = exit;
                let broadcaster_clone = broadcaster.clone();
                
                //Stream through ws_reader
                ws_reader
                //Ensure there are listeners
                .take_while(|_| async {(!broadcaster_clone.strong_count() == 0) && exit.is_empty()})
                //Consume stream 
                .for_each(|msg| async {
                    match msg {
                        Ok(msg) => match broadcaster.send(msg){
                            Ok(_audience) => {
                                //TODO: Add Logging
                            },
                            Err(_err) =>{
                                exit.send(());
                            },
                        },
                        Err(_err) => {
                            //TODO: Add Error Handling
                            
                        },
                    };
                }).await;

            })


        })
    }





    pub async fn send(&self, msg: Message) -> Result<(), ()>{
        self.sender.send(msg).await.map_err(|err|todo!())
    }
    pub fn read(&self) -> tokio::sync::broadcast::Receiver<Message>{
        self.reader.subscribe()
    }
    pub fn close(mut self) -> Result<(), ()>{
        self._exit.send(()).map(|_|()).map_err(|_|{()})
    }
}