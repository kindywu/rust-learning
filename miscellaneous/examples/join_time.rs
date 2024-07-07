use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 共享状态，用于通知线程退出
    let running = Arc::new(Mutex::new(true));
    let r = running.clone();

    // 创建一个新的线程
    let handle = thread::spawn(move || {
        while *r.lock().unwrap() {
            println!("线程正在运行...");
            thread::sleep(Duration::from_secs(1));
        }
        println!("线程退出...");
    });

    // 等待一段时间，让线程运行
    thread::sleep(Duration::from_secs(5));

    // 超时，设置共享状态为false，通知线程退出
    {
        let mut running = running.lock().unwrap();
        *running = false;
    }

    // 等待线程完成
    handle.join().unwrap();
}
