use tokio::{io, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Listening on '127.0.0.1:6142");
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepting conection from: '{}'", addr);
        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("Failed to copy");
            }
        });
    }

    // Ok(())
}
