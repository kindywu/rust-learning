use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake};
use std::thread::{self, Thread};

struct Coroutine<T> {
    state: Arc<Mutex<CoroutineState<T>>>,
}

enum CoroutineState<T> {
    NotStarted(Box<dyn FnOnce() -> T + Send + 'static>),
    Running,
    Completed(T),
}

impl<T> Coroutine<T> {
    fn new<F>(f: F) -> Self
    where
        F: 'static + FnOnce() -> T + Send,
    {
        Coroutine {
            state: Arc::new(Mutex::new(CoroutineState::NotStarted(Box::new(f)))),
        }
    }
}

impl<T> Future for Coroutine<T>
where
    T: Send + 'static,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        match &mut *state {
            CoroutineState::NotStarted(f) => {
                let f = std::mem::replace(f, Box::new(|| panic!("Coroutine already started")));
                *state = CoroutineState::Running;

                // Create a waker to wake up the task
                let waker = cx.waker().clone();
                let state = Arc::clone(&self.state);

                std::thread::spawn(move || {
                    let result = f();
                    let mut state = state.lock().unwrap();
                    *state = CoroutineState::Completed(result);
                    waker.wake();
                });

                Poll::Pending
            }
            CoroutineState::Running => Poll::Pending,
            CoroutineState::Completed(_) => {
                if let CoroutineState::Completed(result) =
                    std::mem::replace(&mut *state, CoroutineState::Running)
                {
                    Poll::Ready(result)
                } else {
                    panic!("CoroutineState changed unexpectedly")
                }
            }
        }
    }
}

struct ThreadWaker(Thread);
impl Wake for ThreadWaker {
    // 实现 wake 方法，当唤醒时调用线程的 unpark 方法
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

struct CoroutineScheduler {}

impl CoroutineScheduler {
    fn schedule<F>(coroutine: F) -> Receiver<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send,
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut coroutine = pin!(coroutine);
            let t = thread::current();
            let waker = Arc::new(ThreadWaker(t)).into();
            let mut cx = Context::from_waker(&waker);
            loop {
                match coroutine.as_mut().poll(&mut cx) {
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
}

fn main() {
    let c = Coroutine::new(|| 100);
    let r = CoroutineScheduler::schedule(c);
    let h = thread::spawn(move || {
        let v = r.recv().unwrap();
        println!("Finish 1 {}, value is {v}", chrono::Utc::now());
    });
    h.join().unwrap();
}
