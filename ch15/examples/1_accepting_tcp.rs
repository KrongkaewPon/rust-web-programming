use std::time;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    while let Ok((stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        tokio::spawn(async move {
            println!("thread starting {} starting", peer.to_string());
            let five_seconds = time::Duration::from_secs(5);
            let begin = time::Instant::now();
            tokio::time::sleep(five_seconds).await;
            let end = begin.elapsed();
            println!(
                "thread {} finishing {}",
                peer.to_string(),
                end.as_secs_f32()
            );
            println!("\n");
        });
    }
}
