use std::{
    future::Future,
    pin::{pin, Pin},
    sync::{
        mpsc::{self, Receiver},
        Arc,
    },
    task::{Context, Poll, Wake},
    thread::{self, sleep, Thread},
    time::{Duration, Instant},
};

use chrono::Local;

// https://www.tisonkun.org/2023/11/05/async-rust/
// https://docs.rs/pollster/latest/pollster/

fn main() {
    block_on_simple();

    block_on_sleep_wakeup();

    spawn_simple();
}

fn spawn_simple() {
    println!("Start {}", chrono::Utc::now());

    let r1 = spawn(MyFuture {
        id: 1,
        start: Instant::now(),
        duration: Duration::from_secs(10),
    });
    let r2 = spawn(MyFuture {
        id: 2,
        start: Instant::now(),
        duration: Duration::from_secs(10),
    });

    let h1 = thread::spawn(move || {
        r1.recv().unwrap();
        println!("Finish 1 {}", chrono::Utc::now());
    });
    let h2 = thread::spawn(move || {
        r2.recv().unwrap();
        println!("Finish 2 {}", chrono::Utc::now());
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

fn block_on_sleep_wakeup() {
    // 调用 block_on 函数来阻塞执行异步任务，存在多次休眠和唤醒操作
    println!("Start {}", Local::now());
    block_on(MyFuture {
        id: 1,
        start: Instant::now(),
        duration: Duration::from_secs(10),
    });
    println!("Finish {}", Local::now());
}

fn block_on_simple() {
    // 调用 block_on 函数来阻塞执行异步任务，并打印 "Hello world"
    block_on(async { println!("Hello world") });
}

struct MyFuture {
    id: u32,
    start: Instant,
    duration: Duration,
}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling {} {}", self.id, Local::now());

        let now = Instant::now();
        let expect = self.start + self.duration;
        let id = self.id;

        if expect > now {
            let duration = expect - now;

            let waker = cx.waker().clone();
            thread::spawn(move || {
                sleep(duration / 2);
                println!("Wake up {id} {}", Local::now());
                waker.wake(); // 关键代码
            });

            println!("Pending {id} {}", Local::now());
            Poll::Pending
        } else {
            println!("Ready {id} {}", Local::now());
            Poll::Ready(())
        }
    }
}

// impl rumtime

// 定义一个实现 Wake trait 的结构体 ThreadWaker，用于唤醒线程
struct ThreadWaker(Thread);
impl Wake for ThreadWaker {
    // 实现 wake 方法，当唤醒时调用线程的 unpark 方法
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

// 定义 block_on 函数，用于在当前线程阻塞直到 Future 执行完成并返回结果
fn block_on<F: Future>(fut: F) -> F::Output {
    // 使用 pin! 宏来将 fut 固定在内存中，以便可以安全地进行异步操作
    let mut fut = pin!(fut);

    // 获取当前线程
    let t = thread::current();

    // 创建一个 Arc 包裹的 ThreadWaker 实例，将其转换为 Waker
    let waker = Arc::new(ThreadWaker(t)).into();

    // 创建一个 Context 对象，用于传递给 Future 的 poll 方法
    let mut cx = Context::from_waker(&waker);

    // 循环执行 Future 的 poll 方法，直到 Future 返回 Ready
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(res) => return res, // 如果 Future 返回 Ready，直接返回结果
            Poll::Pending => thread::park(), // 如果 Future 返回 Pending，当前线程进入阻塞状态
        }
    }
}

// https://docs.rs/glommio/latest/glommio/
// thread per task
fn spawn<F>(fut: F) -> Receiver<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut fut = pin!(fut);
        let t = thread::current();
        let waker = Arc::new(ThreadWaker(t)).into();
        let mut cx = Context::from_waker(&waker);
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(res) => {
                    let _ = tx.send(res);
                    break;
                }
                Poll::Pending => thread::park(),
            }
        }
    });
    rx
}
