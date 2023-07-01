#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Simulate an external source of events that are read in the main
    // loop and acted upon.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);
    tokio::spawn(sender(tx));

    // Store the JoinHandles for the long running tasks.  Indexes is
    // used to get around borrowing issues inside loops.  The compiler can't
    // infer the type of handles hence the type annotation.
    let mut handles: Vec<tokio::task::JoinHandle<Result<u64, String>>> = Vec::new();
    let mut indexes = Vec::new();

    // Loop forever or our simulated external event source send QUIT.
    loop {
        // Get all the handles that have finished.
        for (idx, handle) in handles.iter().enumerate() {
            if handle.is_finished() {
                indexes.push(idx);
            }
        }

        // For each handle that is finished: get a Result by calling await
        // then match to get the actual result or handle the error.  Drain()
        // empties the indexes vector ahead of the next time round the loop.
        for idx in indexes.drain(0..) {
            let handle = handles.remove(idx);
            let result = handle.await?;
            match result {
                Ok(result) => println!("Result: {result}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }

        // Process any events we might have received from our simulated
        // external source.
        match rx.try_recv() {
            // Handle actual events, these are just Strings in this simple
            // example.
            Ok(message) => match message.as_str() {
                "CREATE" => {
                    // Remove this check if you want multiple long running
                    // tasks simultaneously.
                    if handles.len() > 0 {
                        eprintln!("LR already running");
                        continue;
                    }
                    // Spawn a long running task and then push it's JoinHandle
                    // onto the vector.
                    println!("Creating a long running task");
                    let res = tokio::spawn(long_running());
                    handles.push(res);
                }
                "QUIT" => {
                    // Try to be nice and abort all the inflight tasks.
                    // However, this doesn't do anything as our tasks are
                    // sleeping rather than working.
                    for handle in handles {
                        handle.abort();
                    }
                    println!("Got quit message, breaking out of loop");
                    break;
                }
                "STATUS" => {
                    // Because of the check in CREATE, there should only ever
                    // be one task running at a time.
                    println!("There is {} long running task", handles.len());
                }
                // If we get here something very bad has happened.
                m => {
                    eprintln!("Got unknown message {m}");
                }
            },
            // No events to handle, go round the loop again.
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => continue,
            // The simulated external event source has gone away, let's exit.
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => break,
        }
    }

    println!("Exiting");
    Ok(())
}

// Simulate a long running task by sleeping for 15 seconds.
async fn long_running() -> Result<u64, String> {
    println!("LR is starting");
    std::thread::sleep(std::time::Duration::from_secs(15));
    println!("LR is returning");
    // Choose between returning a value or an error by uncommenting one
    // of the following two lines.
    Err("This is an error".to_string())
    // Ok(4321)
}

// Simulated external event source.  Sends a bunch of messages, waiting for
// a few seconds each time.  Change the messages that are sent and the sleep
// times to test different behaviors.
async fn sender(tx: tokio::sync::mpsc::Sender<String>) {
    std::thread::sleep(std::time::Duration::from_secs(2));
    let _ = tx.send("CREATE".to_string()).await;

    let _ = tx.send("STATUS".to_string()).await;

    std::thread::sleep(std::time::Duration::from_secs(2));
    let _ = tx.send("CREATE".to_string()).await;

    let _ = tx.send("STATUS".to_string()).await;

    std::thread::sleep(std::time::Duration::from_secs(20));
    let _ = tx.send("QUIT".to_string()).await;
}
