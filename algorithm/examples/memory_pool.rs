// #![allow(unused)]

use std::{
    alloc::{alloc, dealloc, Layout},
    ptr,
};

struct MemoryPool<T: Default> {
    pool: Vec<*mut T>, // 用于存储空闲内存块的指针
}

impl<T: Default> MemoryPool<T> {
    // 构造函数，初始化内存池
    fn new(size: usize) -> Self {
        let mut pool = Vec::with_capacity(size);
        for _ in 0..size {
            let pointer = unsafe {
                let layout = Layout::new::<T>();
                alloc(layout) as *mut T
            };
            unsafe {
                ptr::write(pointer, T::default());
            }
            pool.push(pointer);
        }
        Self { pool }
    }

    // 分配内存的方法，返回一个指向 T 的可选指针
    fn allocate(&mut self) -> Option<*mut T> {
        self.pool.pop()
    }

    // 回收内存的方法，将指针重新放回内存池
    fn deallocate(&mut self, pointer: *mut T, reset: bool) {
        if reset {
            unsafe {
                ptr::write(pointer, T::default());
            }
        }

        self.pool.push(pointer);
    }

    fn remain(&self) -> usize {
        self.pool.len()
    }
}

// 实现 Drop 特性，以便在内存池销毁时释放所有内存
impl<T: Default> Drop for MemoryPool<T> {
    fn drop(&mut self) {
        while let Some(ptr) = self.pool.pop() {
            unsafe {
                let layout = Layout::new::<T>();
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut pool = MemoryPool::<String>::new(10);

    let ptr1 = pool.allocate().unwrap();
    let ptr2 = pool.allocate().unwrap();

    println!("remain: {}", pool.remain());

    // 使用内存指针写入值
    unsafe {
        ptr::write(ptr1, "hello".to_string());
        ptr::write(ptr2, "world".to_string());
    }

    // 验证写入的值
    unsafe {
        println!("Value at ptr1: {}", *ptr1);
        println!("Value at ptr2: {}", *ptr2);
    }

    // pool.deallocate(ptr1, true);
    // pool.deallocate(ptr2, true);

    pool.deallocate(ptr1, false);
    pool.deallocate(ptr2, false);

    let ptr1 = pool.allocate().unwrap();
    let ptr2 = pool.allocate().unwrap();
    let ptr3 = pool.allocate().unwrap();

    // 验证写入的值
    unsafe {
        println!("Value at ptr1: {}", *ptr1);
        println!("Value at ptr2: {}", *ptr2);
        println!("Value at ptr3: {}", *ptr3);
    }
}
