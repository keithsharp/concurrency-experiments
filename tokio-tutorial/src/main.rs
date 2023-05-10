use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }

    // Ok(())
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    match connection.read_frame().await {
        Ok(frame) => {
            println!("FRAME: {:?}", frame);
            match connection
                .write_frame(&Frame::Error("Unimplemented".to_string()))
                .await
            {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
