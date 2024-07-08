// 实现一个简单的Rust协程库，支持协程的创建、调度和切换。你的实现应该能够处理协程的挂起和恢复，以及协程之间的数据传递。

use rand::seq::SliceRandom;
use std::{
    sync::mpsc::{self, Sender},
    thread::{self, Builder},
};

fn main() {
    let scheduler = CoroutineScheduler::new(4);
    for i in 0..10 {
        let c = Coroutine::new(move || i, &scheduler);
        let result = scheduler.schedule(c);
        println!("运行结束，结果：{}", result);
    }
}

struct Msg<T, F>
where
    F: FnOnce() -> T,
{
    f: F,
    sender: oneshot::Sender<T>,
}

struct CoroutineScheduler<T, F>
where
    F: FnOnce() -> T,
{
    sender_list: Vec<Sender<Msg<T, F>>>,
}

impl<T, F> CoroutineScheduler<T, F>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    fn new(pool_size: u8) -> Self {
        let mut sender_list = Vec::new();
        for i in 0..pool_size {
            let (tx, rx) = mpsc::channel::<Msg<T, F>>();
            let builder = Builder::new().name(format!("worker thread {i}"));
            builder
                .spawn(move || {
                    for msg in rx {
                        println!("{} handle it", thread::current().name().unwrap());
                        let result = (msg.f)();
                        msg.sender
                            .send(result)
                            .unwrap_or_else(|e| eprintln!("Failed to send result: {}", e));
                    }
                })
                .unwrap();
            sender_list.push(tx);
        }

        Self { sender_list }
    }

    fn schedule(&self, c: Coroutine<T, F>) -> T
    where
        T: Send + 'static,
    {
        c.resume()
    }

    fn execute(&self, f: F) -> T {
        let (tx, rx) = oneshot::channel();

        let msg = Msg { f, sender: tx };

        let mut rng = rand::thread_rng();
        let tx = self
            .sender_list
            .choose(&mut rng)
            .expect("Sender list is empty");
        tx.send(msg).unwrap();

        rx.recv().unwrap_or_else(|e| {
            eprintln!("Failed to receive result: {}", e);
            panic!("Coroutine execution failed");
        })
    }
}

struct Coroutine<'a, T, F>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    func: F,
    scheduler: &'a CoroutineScheduler<T, F>,
}

impl<'a, T, F> Coroutine<'a, T, F>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    fn new(func: F, scheduler: &'a CoroutineScheduler<T, F>) -> Self {
        Coroutine { func, scheduler }
    }

    fn resume(self) -> T {
        self.scheduler.execute(self.func)
    }
}
