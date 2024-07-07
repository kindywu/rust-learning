// 使用Rust的线程同步机制，实现一个线程安全的单向链表。链表应该支持线程安全的插入和删除操作。

// use std::sync::{Arc, Mutex};

// struct Node<T> {
//     value: T,
//     next: Option<Arc<Mutex<Node<T>>>>,
// }

// struct ThreadSafeList<T> {
//     head: Option<Arc<Mutex<Node<T>>>>,
// }

// impl<T> ThreadSafeList<T> {
//     // 实现构造函数
//     fn new() -> Self {
//         // 你的代码
//     }

//     // 实现插入元素的方法
//     fn insert(&mut self, value: T) {
//         // 你的代码
//     }

//     // 实现删除元素的方法
//     fn remove(&mut self) -> Option<T> {
//         // 你的代码
//     }
// }

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() -> anyhow::Result<()> {
    println!("sync");
    sync()?;
    println!("concurrency");
    concurrency()?;
    Ok(())
}

// for debug
fn sync() -> anyhow::Result<()> {
    let mut list = ThreadSafeList::<u32>::new();

    list.insert(77);
    list.insert(88);
    list.insert(99);

    println!("{:?}", list.remove());
    println!("{:?}", list.remove());
    println!("{:?}", list.remove());

    println!("{:?}", list.remove());

    println!("{list:+?}");

    Ok(())
}

fn concurrency() -> anyhow::Result<()> {
    let list = ThreadSafeList::new();

    let mut handles = Vec::new();
    for i in 0..1000 {
        let mut list = list.clone();
        let handle = thread::spawn(move || {
            list.insert(i);
            list.insert(i);
            list.insert(i);
            list.remove();
            list.remove();
            list.remove();

            list.remove();

            list.insert(i);
            list.remove();

            list.remove();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{list:+?}");

    Ok(())
}

#[derive(Debug)]
struct Node<T: Default + Copy> {
    value: T,
    next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T: Default + Copy> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn to_option_arc_mutex(self) -> Option<Arc<Mutex<Node<T>>>> {
        Some(Arc::new(Mutex::new(self)))
    }
}

#[derive(Debug, Clone)]
struct ThreadSafeList<T: Default + Copy> {
    head: Option<Arc<Mutex<Node<T>>>>,
}

impl<T: Default + Copy> ThreadSafeList<T> {
    // 实现构造函数
    fn new() -> Self {
        let head = Node::new(Default::default()).to_option_arc_mutex();
        Self { head }
    }

    // 实现插入元素的方法
    fn insert(&mut self, value: T) {
        let current = self.head.clone();
        if current.is_none() {
            unreachable!()
        }

        let mut current: Arc<Mutex<Node<T>>> = current.unwrap();

        loop {
            let tmp = current.clone();
            let mut tmp = tmp.lock().unwrap();
            let next = tmp.next.clone();
            if let Some(next) = next {
                current = next;
            } else {
                tmp.next = Node::new(value).to_option_arc_mutex();
                break;
            }
        }
    }

    // 实现删除元素的方法
    fn remove(&mut self) -> Option<T> {
        let current = self.head.clone();
        if current.is_none() {
            unreachable!()
        }

        let mut current: Arc<Mutex<Node<T>>> = current.unwrap();

        let value = loop {
            // 重点：tmp == current，tmp2 == next，使用克隆变量tmp，而不用current和next是因为调用lock().unwrap会把变量消费掉
            let tmp = current.clone();
            let mut tmp = tmp.lock().unwrap();

            if tmp.next.is_none() {
                // 处理头节点
                return None;
            }

            let next = tmp.next.clone().unwrap();

            let tmp2 = next.clone();
            let tmp2 = tmp2.lock().unwrap();
            let next_next = tmp2.next.clone();

            // 找到倒数第二个的节点，删除尾节点，设置它的Next为None，返回尾节点的值
            if next_next.is_none() {
                tmp.next.take();
                let tmp = tmp2.value;
                break tmp;
            }

            // current.next.next 不为None，移动到下个节点
            current = next;
        };
        Some(value)
    }
}
