use std::sync::{Arc, Mutex};
use std::thread;
const MSG: &str = "Failed to acquire lock";
fn main() {
    // 创建一个共享的计数器
    let counter = Arc::new(Mutex::new(0));

    // 创建多个线程来增加计数器的值
    let mut handles = vec![];

    for _ in 0..1000 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取 Mutex 的锁
            let mut num = counter.lock().expect(MSG);

            // 在临界区内修改计数器的值
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终的计数器值
    println!("Final counter value: {:?}", *counter.lock().expect(MSG));
}
