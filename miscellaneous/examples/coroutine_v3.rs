use rand::seq::SliceRandom;
use std::{
    sync::mpsc::{self, Sender},
    thread,
};
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
            let builder = thread::Builder::new().name(format!("worker thread {i}"));
            builder
                .spawn(move || {
                    for msg in rx {
                        println!("{:?} handle it", thread::current().name());
                        let f = msg.f;
                        let result = f();
                        msg.sender.send(result).unwrap();
                    }
                })
                .unwrap();
            sender_list.push(tx);
        }

        Self { sender_list }
    }

    fn schedule(&self, f: F) -> T
    where
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let msg = Msg::<T, F> { f: f, sender: tx };

        {
            let mut rng = rand::thread_rng();
            let tx = self.sender_list.choose(&mut rng).expect("arr is empty");
            tx.send(msg).unwrap();
        }

        rx.recv().unwrap()
    }
}

fn main() {
    let scheduler = CoroutineScheduler::new(4);
    for i in 0..10 {
        let result = scheduler.schedule(move || i);
        println!("运行结束，结果：{}", result);
    }
}
