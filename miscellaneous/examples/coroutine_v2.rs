use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

struct Coroutine<T> {
    result: Arc<(Mutex<Option<T>>, Condvar)>,
    handle: Option<JoinHandle<()>>,
}

impl<T> Coroutine<T> {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let result = Arc::new((Mutex::new(None), Condvar::new()));
        let result_clone = result.clone();

        let handle = thread::spawn(move || {
            let (lock, cvar) = &*result_clone;
            let result_value = f();
            let mut result_guard = lock.lock().unwrap();
            *result_guard = Some(result_value);
            cvar.notify_one();
        });

        Coroutine {
            result,
            handle: Some(handle),
        }
    }

    fn resume(mut self) -> T {
        let (lock, cvar) = &*self.result;
        let mut result_guard = lock.lock().unwrap();
        while result_guard.is_none() {
            result_guard = cvar.wait(result_guard).unwrap();
        }
        self.handle.take().unwrap().join().unwrap();
        result_guard.take().unwrap()
    }
}

struct CoroutineScheduler;

impl CoroutineScheduler {
    fn schedule<T>(&self, coroutine: Coroutine<T>) -> T {
        coroutine.resume()
    }
}

fn main() {
    let scheduler = CoroutineScheduler;
    let coroutine = Coroutine::new(|| {
        println!("测试测试!");
        1
    });
    let result = scheduler.schedule(coroutine);
    println!("运行结束: {}", result);
}
