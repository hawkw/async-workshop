use tokio_executor::spawn;
use tokio_executor::thread_pool::Builder;
use tokio_sync::{mpsc, oneshot};

use std::thread;
use std::time::Duration;

fn main() {
    let mut b = Builder::new();
    b.threads(1);
    println!("{:?}", b);

    let rt = b.build();

    let (mut tx, mut rx) = mpsc::unbounded_channel();
    let (start_tx, start_rx) = oneshot::channel();

    rt.spawn(async move {
        println!(" -> hello!");

        spawn(async move {
            println!(" -> Two!");

            start_tx.send(()).unwrap();

            rx.recv().await;
            println!(" -> recv 1");

            rx.recv().await;
            println!(" -> recv 2");
        });
    });

    let mut e = tokio_executor::enter().unwrap();
    e.block_on(start_rx).unwrap();

    println!(" -> send 1");
    tx.try_send(()).unwrap();
    thread::sleep(Duration::from_secs(1));

    println!(" -> send 2");
    tx.try_send(()).unwrap();
    thread::sleep(Duration::from_secs(10));
}
