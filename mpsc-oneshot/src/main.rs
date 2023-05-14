use std::thread::sleep;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::oneshot;

use tokio::task;

use uuid::Uuid;

#[derive(Debug)]
enum Event {
    Work(Work),
    Quit,
}

#[derive(Debug)]
struct Work {
    id: Uuid,
    amount: u64,
}

#[derive(Debug)]
enum Response {
    Complete(Uuid),
    Quitting,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (m_tx, m_rx) = mpsc::channel::<(Event, oneshot::Sender<Response>)>(100);

    let handle = task::spawn_blocking(move || {
        perform_work(m_rx);
    });

    let work = Work {
        id: Uuid::new_v4(),
        amount: 5,
    };

    println!("Sending work.");
    let (o_tx, o_rx) = oneshot::channel();
    m_tx.send((Event::Work(work), o_tx)).await?;

    let response = o_rx.await?;
    match response {
        Response::Complete(id) => println!("Got response ID: {}", id),
        _ => (),
    }

    println!("Sending quit.");
    let (o_tx, o_rx) = oneshot::channel();
    m_tx.send((Event::Quit, o_tx)).await?;

    let response = o_rx.await?;
    match response {
        Response::Quitting => println!("Work loop is quitting."),
        _ => (),
    }

    handle.await?;

    println!("Exiting program.");
    Ok(())
}

fn perform_work(mut rx: mpsc::Receiver<(Event, oneshot::Sender<Response>)>) {
    loop {
        match rx.try_recv() {
            Ok((event, o_tx)) => match event {
                Event::Work(work) => {
                    println!("Performing '{}' for {} seconds", work.id, work.amount);
                    sleep(Duration::from_secs(work.amount));
                    o_tx.send(Response::Complete(work.id)).unwrap();
                }
                Event::Quit => {
                    println!("Perform work got quit message.");
                    o_tx.send(Response::Quitting).unwrap();
                    break;
                }
            },
            Err(TryRecvError::Empty) => {
                println!("No work to do, sleeping for 0.5 seconds");
                sleep(Duration::from_millis(500));
            }
            Err(TryRecvError::Disconnected) => {
                println!("Got disconnected from all MPSC senders.");
                break;
            }
        }
    }
}
