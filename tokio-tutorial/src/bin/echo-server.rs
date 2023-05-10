use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Listening on '127.0.0.1:6142");
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepting conection from: '{}'", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Closing connection to: '{}'", addr);
                        return;
                    }
                    Ok(n) => match socket.write_all(&buf[..n]).await {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Write error: {}", e);
                            return;
                        }
                    },
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        return;
                    }
                }
            }
        });
    }

    // Ok(())
}
