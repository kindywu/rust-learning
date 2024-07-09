use tokio::{select, sync::mpsc::channel};
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    channel_biased().await;
    stream_biased().await;
}

async fn channel_biased() {
    let (tx1, mut rx1) = channel::<()>(1);
    drop(tx1);
    let (tx2, mut rx2) = channel::<()>(1);
    drop(tx2);

    let mut ready1 = 0;
    let mut ready2 = 0;

    let n = 100;
    for _ in 0..n {
        select! {
            biased; // 按顺序优先级，注释掉则是随机优先级；
            _ = rx1.recv()=>{
                ready1+=1;
            },
            _ = rx2.recv()=>{
                ready2+=1;
            }
        }
    }

    // println!("ready1 = {ready1} ready2 = {ready2}");

    assert_eq!(ready1, n);
    assert_eq!(ready2, 0);
}

async fn stream_biased() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let mut values = vec![];

    loop {
        tokio::select! {
            biased;// 按顺序优先级，注释掉则是随机优先级，Vector是乱序
            Some(v) = stream1.next() => values.push(v),
            Some(v) = stream2.next() => values.push(v),
            else => break,
        }
    }

    // println!("{:?}", values);
    assert_eq!(&[1, 2, 3, 4, 5, 6], &values[..]);
}
