use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};

const SERVER_ADDR:&str = "127.0.0.1:8001";

#[tokio::main]
async fn main() {

    println!("Starting Server at {}", SERVER_ADDR);

    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();

    loop{
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move{
            handle_connection(stream).await;

        });

    }
}

async fn handle_connection(mut Stream:TcpStream){
    let mut buffer = [0;1024];
    let len = Stream.read(&mut buffer).await.unwrap();

    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}", message);

    let _ = Stream.write_all(&message.as_bytes()).await;
    println!("Sent: {}", message);
}