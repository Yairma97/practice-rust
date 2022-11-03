mod noise_codec;
mod pb;

use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use noise_codec::{NoiseCodec, NOISE_PARAMS,NoiseStream};
use pb::*;
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Client Starting");
    let addr = "127.0.0.1:8888";
    let stream = TcpStream::connect(addr).await?;
    let mut stream = NoiseCodec::builder(NOISE_PARAMS, true).new_framed(stream)?;
    // -> e
    stream.send(Bytes::from_static(&[])).await?;
    info!("-> e");

    // <- e, ee, s, es
    let data = stream.next().await.unwrap()?;
    info!("<- e, ee, s, es");

    // -> s, se
    stream.send(data.freeze()).await?;
    info!("-> s, se");

    stream.into_transport_mode()?;



    let msg = Request::new_put("Hello", b"Buffer");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("Hello");
    stream.send(msg.into()).await?;

    let msg = Request::key_del("Hello");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("Hello");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        let msg = Response::try_from(buf)?;
        println!("Got msg :{:?}", msg);
    }
    Ok(())
}
