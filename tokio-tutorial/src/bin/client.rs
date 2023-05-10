use bytes::Bytes;
use tokio::sync::mpsc;

// #[derive(Debug)]
// enum Command {
//     Get { key: String },
//     Set { key: String, value: Bytes },
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("Message from Task ONE")
            .await
            .expect("message ONE should always send");
    });

    tokio::spawn(async move {
        tx2.send("Message from Task TWO")
            .await
            .expect("message TWO should always send");
    });

    while let Some(message) = rx.recv().await {
        println!("Got message: {}", message);
    }

    Ok(())
}
