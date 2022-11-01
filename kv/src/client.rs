mod pb;

use anyhow::Result;
use pb::*;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Client Starting");
    let addr = "127.0.0.1:8888";
    let stream = TcpStream::connect(addr).await?;
    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);
    let msg = Request::new_put("Hello",b"Buffer");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("Hello");
    stream.send(msg.into()).await?;

    let msg = Request::key_del("Hello");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("Hello");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await{
        let msg = Response::try_from(buf)?;
        println!("Got msg :{:?}",msg);

    }
    Ok(())
}
