mod pb;
mod noise_codec;
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use pb::{request::*, *};
use tokio::net::TcpListener;
use noise_codec::{NOISE_PARAMS,NoiseCodec,NoiseStream};
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}
impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Server Starting");

    let state = Arc::new(ServerState::new());
    let addr = "127.0.0.1:8888";
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening to {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("New client: {:?} accepted", addr);
        let shared = state.clone();
        tokio::spawn(async move {
            let mut stream = NoiseCodec::builder(NOISE_PARAMS,false).new_framed(stream)?;

            // <- e
            let data = stream.next().await.unwrap()?;
            info!("<- e");

            // -> e, ee, s, es
            stream.send(data.freeze()).await?;
            info!(" -> e, ee, s, es");
            
            // <- s, se
            let _data = stream.next().await.unwrap();
            info!("<- s, se");
        
            stream.into_transport_mode()?;

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a command from {:?}", msg);

                let response: Response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.store.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    Some(Command::Del(RequestDel { key })) => 
                        match shared.store.contains_key(&key){
                            true => {
                                shared.store.remove(&key);
                                info!("{} has been deleted", &key);
                                Response::del_key(key)
                            },
                            false => Response::not_found(key),
                    }
                    None => unimplemented!(),
                };
                stream.send(response.into()).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}
