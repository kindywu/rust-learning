#![allow(unused)]

/*
 实现一个简单的Rust协程库，支持协程的创建、调度和切换。你的实现应该能够处理协程的挂起和恢复，以及协程之间的数据传递。
*/
use std::sync::{Arc, Mutex};

struct Coroutine {
    task: Mutex<Box<dyn FnMut() + Send + 'static>>,
    finished: Mutex<bool>,
}

impl Coroutine {
    fn new(task: impl FnMut() + Send + 'static) -> Arc<Self> {
        Arc::new(Coroutine {
            task: Mutex::new(Box::new(task)),
            finished: Mutex::new(false),
        })
    }

    fn run(self: Arc<Self>) {
        let mut task = self.task.lock().unwrap();
        let mut finished = self.finished.lock().unwrap();
        if !*finished {
            task();
            *finished = true;
        }
    }

    fn is_finished(&self) -> bool {
        *self.finished.lock().unwrap()
    }
}
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;

struct Scheduler {
    tx: Sender<Arc<Coroutine>>,
    handle: JoinHandle<()>,
}

impl Scheduler {
    fn new() -> Self {
        let (tx, rx) = channel::<Arc<Coroutine>>();
        let handle = thread::spawn(move || {
            while let Ok(coroutine) = rx.recv() {
                coroutine.run();
            }
            println!("scheduler loop quit")
        });
        Self { tx, handle }
    }

    fn spawn(&self, task: impl FnMut() + Send + 'static) {
        let coroutine = Coroutine::new(task);
        self.tx.send(coroutine);
    }

    fn join(self, timeout: Option<Duration>) {
        match timeout {
            Some(timeout) => {
                sleep(timeout);
                drop(self.tx);
            }
            None => {
                self.handle.join();
            }
        }
    }
}

fn main() {
    let scheduler = Scheduler::new();

    scheduler.spawn(|| {
        for i in 1..=5 {
            println!("Coroutine 1: {}", i);
        }
    });

    scheduler.spawn(|| {
        for i in 1..=5 {
            println!("Coroutine 2: {}", i);
        }
    });

    scheduler.join(Some(Duration::from_secs(5)))
}
