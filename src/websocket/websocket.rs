
#[derive(Debug)]
pub enum WebsocketMessage {
    UpdateSongData(SongData),
    Clear
}

#[derive(Serialize, Debug, Clone)]
pub struct SongData {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub b64img: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<bool>
}

use iced::futures::channel::mpsc::Receiver;
use serde::Serialize;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

use std::{io::Error, sync::{Arc, Mutex}};

use futures_util::{SinkExt, StreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver}};

use crate::config;

pub async fn ws_main(reciever: Receiver<WebsocketMessage>) -> Result<(), Error> {

    let song_data: Arc<Mutex<SongData>> = Arc::new(Mutex::new(SongData {
        title: String::new(),
        artist: String::new(),
        album: String::new(),
        b64img: String::new(),
        clear: Some(true)
    }));

    let port = config::PROGRAM_CFG.ws_port();

    let addr = format!("127.0.0.1:{}", port);

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("bind failed");

    let (tx, _) = tokio::sync::broadcast::channel::<SongData>(10);
    tokio::spawn(song_listener(song_data.clone(), reciever, tx.clone()));

    while let Ok((stream, _)) = listener.accept().await {
        let rx = tx.subscribe();
        tokio::spawn(accept_connection(stream, rx, song_data.clone()));
    }

    Ok(())
}

async fn song_listener(song_data: Arc<Mutex<SongData>>, mut receiver: Receiver<WebsocketMessage>, tx: BroadcastSender<SongData>) {
    loop {
        let mut final_msg = receiver.next().await;
        // discard all but the final message
        while let Ok(msg) = receiver.try_next() {
            final_msg = Some(msg.unwrap());
        }

        if final_msg.is_none() {
            continue;
        }

        let final_msg = final_msg.unwrap();

        match final_msg {
            WebsocketMessage::UpdateSongData(data) => {
                let mut guard = song_data.lock().unwrap();
                *guard = data;
                _ = tx.send(guard.clone());
                drop(guard);
            },
            WebsocketMessage::Clear => {
                let mut guard = song_data.lock().unwrap();
                *guard = SongData {
                    title: String::new(),
                    artist: String::new(),
                    album: String::new(),
                    b64img: String::new(),
                    clear: Some(true)
                };
                _ = tx.send(guard.clone());
                drop(guard);

            }
        }

    }  
}

async fn accept_connection(stream: TcpStream, mut rx: BroadcastReceiver<SongData>, song_mutex: Arc<Mutex<SongData>>) {

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("handshake err");

    let (mut write, _) = ws_stream.split();
    let d;
    {
        let guard = song_mutex.lock().unwrap();
        d = guard.clone();
        drop(guard);
    }
    let j = json!(d).to_string();
    let message = Message::Text(j.into());
    _ = write.send(message).await;

    loop {
        let data = rx.recv().await;

        if data.is_err() {continue};
        let data = data.unwrap();
        let j = json!(data).to_string();
        let message = Message::Text(j.into());
        let res = write.send(message).await;
        if res.is_err() {
            break;
        }
    }
}