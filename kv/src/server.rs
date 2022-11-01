mod pb;

use std::sync::Arc;

use dashmap::DashMap;
use pb::*;
use prost::Message;
use tokio::net::{TcpListener,TcpStream};
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;
use futures::StreamExt;

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

    info!("Starting");

    let state = Arc::new(ServerState::new());
    let addr = "127.0.0.1:8888";
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening to {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("New client: {:?} accepted", addr);
        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)
                .new_framed(stream);
            while let Some(Ok(buf)) = stream.next().await? {
                todo!()
            }
        })
    }
    Ok(())
}
