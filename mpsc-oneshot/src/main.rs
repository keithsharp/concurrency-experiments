use std::thread::sleep;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::oneshot;

use tokio::sync::mpsc::error::TryRecvError;
use tokio::task;

use uuid::Uuid;

#[derive(Debug)]
struct Work {
    id: Uuid,
    amount: u64,
}

#[derive(Debug)]
struct Response(Uuid);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (m_tx, m_rx) = mpsc::channel::<(Work, oneshot::Sender<Response>)>(100);
    let (o_tx, o_rx) = oneshot::channel();

    let handle = task::spawn_blocking(move || {
        perform_work(m_rx);
    });

    let work = Work {
        id: Uuid::new_v4(),
        amount: 5,
    };

    println!("Sending work.");
    m_tx.send((work, o_tx)).await?;

    let response = o_rx.await?;
    println!("Got response ID: {}", response.0);

    handle.await?;

    println!("Shutting down.");
    Ok(())
}

fn perform_work(mut rx: mpsc::Receiver<(Work, oneshot::Sender<Response>)>) {
    loop {
        match rx.try_recv() {
            Ok((work, o_tx)) => {
                println!("Performing '{}' for {} seconds", work.id, work.amount);
                sleep(Duration::from_secs(work.amount));
                o_tx.send(Response(work.id)).unwrap();
            }
            Err(TryRecvError::Empty) => {
                // println!("No work to do, sleeping for 0.5 seconds");
                sleep(Duration::from_millis(500));
            }
            Err(TryRecvError::Disconnected) => {
                println!("Got disconnected from all MPSC senders.");
                break;
            }
        }
    }
}
