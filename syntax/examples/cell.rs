#![allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::{hash_map, HashMap},
};

pub struct Calculator {
    f: Box<dyn Fn(i64) -> i64>,
    // cache: Cell<HashMap<i64, i64>>, //内部可变性
    // cache: RefCell<HashMap<i64, i64>>, //内部可变性
    cache: UnsafeCell<HashMap<i64, i64>>, //内部可变性
}

impl Calculator {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(i64) -> i64 + 'static,
    {
        Self {
            f: Box::new(f),
            // cache: Cell::new(HashMap::new()),
            cache: UnsafeCell::new(HashMap::new()),
        }
    }

    pub fn eval(&self, input: i64) -> i64 {
        // let mut cache = self.cache.take();
        // let mut cache = self.cache.borrow_mut();
        // SAFETY: cache is exclusively assessed in this scope
        let cache = unsafe { &mut *self.cache.get() };

        let value = match cache.entry(input) {
            hash_map::Entry::Occupied(entry) => entry.get().to_owned(),
            hash_map::Entry::Vacant(entry) => {
                let value = (self.f)(input);
                entry.insert(value);
                value
            }
        };
        // self.cache.set(cache);
        value
    }
}

fn main() {
    let cal = Calculator::new(|i| i);
    println!("{}", cal.eval(100))
}

#[cfg(test)]
mod tests {
    use crate::Calculator;

    #[test]
    fn test_calculator() {
        fn fib(n: i64) -> i64 {
            if n <= 2 {
                return n;
            }
            fib(n - 1) + fib(n - 2)
        }

        let cal = Calculator::new(|i| {
            println!("calling fib({})", i);
            fib(i)
        });

        assert_eq!(cal.eval(2), 2);
        assert_eq!(cal.eval(8), 34);
        assert_eq!(cal.eval(8), 34);
        assert_eq!(cal.eval(30), 1346269);
        assert_eq!(cal.eval(30), 1346269);
    }
}
