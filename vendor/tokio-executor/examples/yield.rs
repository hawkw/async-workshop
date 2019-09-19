use tokio_executor::thread_pool::*;

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc;
use std::task::{Context, Poll};

struct Backoff(usize);

impl Future for Backoff {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 0 {
            Poll::Ready(())
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    const NUM_YIELD: usize = 10;
    const TASKS_PER_CPU: usize = 1;

    let threadpool = Builder::new().build();

    let tasks = TASKS_PER_CPU * num_cpus::get_physical();
    let (tx, rx) = mpsc::sync_channel(tasks);

    for _ in 0..tasks {
        let tx = tx.clone();

        threadpool.spawn(async move {
            let backoff = Backoff(NUM_YIELD);
            backoff.await;
            tx.send(()).unwrap();
        });
    }

    for _ in 0..tasks {
        let _ = rx.recv().unwrap();
    }
}
