use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    // thread::scope会自动等待所有线程完成它们的任务
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            sleep(Duration::from_secs(3));
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}
