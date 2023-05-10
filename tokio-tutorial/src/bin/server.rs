use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Arc::new(Mutex::new(HashMap::new()));

    println!("Listening on '127.0.0.1:6379");
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        println!("Accepted socket, processing");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }

    // Ok(())
}

async fn process(socket: TcpStream, db: Db) {
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
                let mut db = db.lock().expect("should be able to acquire lock on db");
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().expect("should be able to acquire lock on db");
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
