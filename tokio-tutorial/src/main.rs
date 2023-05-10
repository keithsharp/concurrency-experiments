use std::collections::HashMap;

use mini_redis::Command::{self, Get, Set};
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
    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);
    while let Some(frame) = connection
        .read_frame()
        .await
        .expect("should always be able to read a frame")
    {
        let response = match Command::from_frame(frame)
            .expect("should always be able to get a command from a frame")
        {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Command  '{:?}' is not implemented", cmd),
        };

        connection
            .write_frame(&response)
            .await
            .expect("should always be able to write response frame");
    }
}
