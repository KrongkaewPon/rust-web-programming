use bincode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.split();

    let message = Message {
        ticker: String::from("BYND"),
        amount: 3.2,
    };
    let message_bin = bincode::serialize(&message).unwrap();

    println!("stream starting");
    writer.write_all(&message_bin).await?;
    writer.write_all(b"\n").await?;
    println!("sent data");
    let mut buf_reader = BufReader::new(reader);

    let mut buf = vec![];
    println!("reading data");
    let _ = buf_reader.read_until(b'\n', &mut buf).await.unwrap();
    println!("{:?}", bincode::deserialize::<Message>(&buf));
    Ok(())
}
