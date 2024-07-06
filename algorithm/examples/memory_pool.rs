struct MemoryPool<T>
where
    T: Default + Clone + Copy,
{
    // 定义内存池的数据结构
    pool: Vec<T>,
}

impl<T> MemoryPool<T>
where
    T: Default + Clone + Copy,
{
    // 实现内存池的构造函数
    fn new() -> Self {
        Self { pool: Vec::new() }
    }

    // 实现分配内存的方法
    fn allocate(&mut self) -> Option<*mut T> {
        let item = if self.pool.len() > 0 {
            self.pool.pop()?
        } else {
            Default::default()
        };

        Some(Box::into_raw(Box::new(item))) //这里需要将T转为*T
    }

    // 实现回收内存的方法
    fn deallocate(&mut self, pointer: *mut T) {
        let item = unsafe { *pointer };
        self.pool.push(item)
    }

    fn len(&self) -> usize {
        self.pool.len()
    }
}

fn main() {
    let mut mp = MemoryPool::<i32>::new();
    let pointer = mp.allocate().unwrap();
    unsafe {
        *pointer = 100;
    }

    unsafe {
        println!("Value at pointer: {}", *pointer);
    }
    mp.deallocate(pointer);

    let mut mp = MemoryPool::<i32>::new();
    let pointer = mp.allocate().unwrap();
    unsafe {
        *pointer = 100;
    }

    unsafe {
        println!("value at pointer: {}, pool len: {} ", *pointer, mp.len());
    }
    mp.deallocate(pointer);

    println!("after deallocate pool len: {}", mp.len());

    let pointer = mp.allocate().unwrap();
    unsafe {
        *pointer = 200;
    }

    unsafe {
        println!("value at pointer: {}, pool len: {} ", *pointer, mp.len());
    }
    mp.deallocate(pointer);

    println!("after deallocate pool len: {}", mp.len());
}
