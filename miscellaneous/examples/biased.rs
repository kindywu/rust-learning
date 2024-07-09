use tokio::{select, sync::mpsc::channel};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = channel::<()>(1);
    drop(tx1);
    let (tx2, mut rx2) = channel::<()>(1);
    drop(tx2);

    let mut ready1 = 0;
    let mut ready2 = 0;

    for _ in 0..100 {
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

    println!("ready1: {}", ready1);
    println!("ready2: {}", ready2);
}
