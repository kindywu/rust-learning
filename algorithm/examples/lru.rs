use std::collections::HashMap;
use std::hash::Hash;
use std::ptr;

pub struct Node<K, V> {
    key: K,
    value: V,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

pub struct LuaCache<K, V> {
    capacity: usize,
    size: usize,
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    map_list: HashMap<K, *mut Node<K, V>>,
}

impl<K: Eq + Hash + Clone, V> LuaCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LuaCache {
            capacity,
            size: 0,
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            map_list: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<*mut Node<K, V>> {
        if let Some(&node) = self.map_list.get(&key) {
            unsafe {
                (*node).value = value;
            }
            self.move_to_head(node);
            return None;
        }

        let node = Box::into_raw(Box::new(Node {
            key: key.clone(),
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }));

        self.map_list.insert(key, node);
        self.add_node_to_head(node);

        if self.size > self.capacity {
            return Some(self.remove_tail_node());
        }

        None
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if let Some(&node) = self.map_list.get(&key) {
            self.move_to_head(node);
            unsafe {
                return Some(&(*node).value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: K) -> bool {
        if let Some(node) = self.map_list.remove(&key) {
            self.remove_node(node);
            return true;
        }
        false
    }

    pub fn info(&self) -> (usize, usize) {
        (self.size, self.capacity)
    }

    fn move_to_head(&mut self, node: *mut Node<K, V>) {
        if self.head == node {
            return;
        }
        self.remove_node(node);
        self.add_node_to_head(node);
    }

    fn add_node_to_head(&mut self, node: *mut Node<K, V>) {
        unsafe {
            (*node).next = self.head;
            (*node).prev = ptr::null_mut();

            if !self.head.is_null() {
                (*self.head).prev = node;
            }

            self.head = node;

            if self.tail.is_null() {
                self.tail = node;
            }
        }

        self.size += 1;
    }

    fn remove_node(&mut self, node: *mut Node<K, V>) {
        unsafe {
            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next;
            } else {
                self.head = (*node).next;
            }

            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev;
            } else {
                self.tail = (*node).prev;
            }
        }

        self.size -= 1;
    }

    fn remove_tail_node(&mut self) -> *mut Node<K, V> {
        if self.tail.is_null() {
            return ptr::null_mut();
        }

        let evicted = self.tail;

        unsafe {
            if !(*evicted).prev.is_null() {
                self.tail = (*evicted).prev;
                (*self.tail).next = ptr::null_mut();
            } else {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            }

            let key = (*evicted).key.clone();
            self.map_list.remove(&key);
        }

        evicted
    }
}

impl<K, V> Drop for LuaCache<K, V> {
    fn drop(&mut self) {
        let mut node = self.head;
        while !node.is_null() {
            unsafe {
                let next = (*node).next;
                let _ = Box::from_raw(node);
                node = next;
            }
        }
    }
}

fn main() {
    let mut lua_cache = LuaCache::new(3);

    lua_cache.put(1, 1);
    println!("{:?}", lua_cache.info());
    lua_cache.put(2, 2);
    lua_cache.put(3, 3);
    lua_cache.put(1, 1);
    println!("{:?}", lua_cache.info());
    lua_cache.remove(1);
    println!("{:?}", lua_cache.info());
    lua_cache.put(4, 4);
    println!("{:?}", lua_cache.info());

    let evicted = lua_cache.put(5, 5);
    assert!(evicted.is_some());
    let evicted = evicted.unwrap();
    // let evicted = unsafe { &*evicted };
    // let evicted = unsafe { &mut *evicted };
    let evicted = unsafe { Box::from_raw(evicted) };
    assert_eq!(2, evicted.key)
}
