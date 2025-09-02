use futures_util::{select, SinkExt, StreamExt};
use tokio::{sync::{broadcast, mpsc::{self}}, task::JoinHandle, try_join};
use tokio_tungstenite::{connect_async, tungstenite::Message};


pub struct Connection{
    _sender: mpsc::Sender<Message>,
    _broadcaster: broadcast::Sender<Message>,
    _handle: JoinHandle<()>,
    _exit: broadcast::Sender<()>
}
impl Connection{


    pub async fn connect(url: &str) -> Result<Self, ()>{
        let (_sender, mut recver) = mpsc::channel::<Message>(32);
        let (_broadcaster, _) = broadcast::channel::<Message>(usize::MAX);
        let (_exit, _) = broadcast::channel::<()>(1);
        
        //Setup connection
        let (mut ws_writer, ws_reader) = connect_async(url).await
        //re-map errors
        .map_err(|_err| {
            todo!()
        })?.0
        //split into r/w
        .split();

        //Clones for move
        let exit = _exit.clone();
        let broadcaster = _broadcaster.clone();

        //Parent Thread (spawn 3 internal threads and manage lifecycles)
        let _handle = tokio::spawn(async move{
            //Clone for write thread
            let _exit = exit.clone();
            //Write Thread
            let writer_handle: JoinHandle<Result<(), ()>> = tokio::task::spawn(async move{
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
                Err(())
            });
    
            //Clone for read thread
            let _exit = exit.clone();
            //Read Thread
            let reader_handle: JoinHandle<Result<(), ()>> = tokio::task::spawn(async move{
                let broadcaster_clone = broadcaster.clone();
                //Stream through ws_reader
                ws_reader
                //Ensure there are listeners
                .take_while(|_| async {(!broadcaster_clone.strong_count() == 0)})
                //Consume stream 
                .for_each(|msg| async {
                    match msg {
                        Ok(msg) => match broadcaster.send(msg){
                            Ok(_audience) => {
                                //TODO: Add Logging
                            },
                            Err(_err) =>{
                                let _ = _exit.send(());
                            },
                        },
                        Err(_err) => {
                            //TODO: Add Error Handling
                            
                        },
                    };
                }).await;
                Err(())
            });

            //Exit Thread
            let exit_handle: JoinHandle<Result<(), ()>> = tokio::spawn(async move {
                let _ = exit.subscribe().recv().await;
                Err(())
            });

            //Join
            let _ = try_join!(reader_handle, writer_handle, exit_handle);
        });

        Ok(Self{
            //Send messages to Kraken API
            _sender,
            //Recieve messages from Kraken API 
            _broadcaster,
            _exit,
            _handle
        })
    }

    pub async fn send(&self, msg: Message) -> Result<(), ()>{
        self._sender.send(msg).await.map_err(|_err|todo!())
    }
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Message>{
        self._broadcaster.subscribe()
    }
    pub fn close(self){
        let _ = self._exit.send(());
    }
}