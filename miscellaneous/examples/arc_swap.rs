use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_utils::thread;

fn main() {
    let config = ArcSwap::from(Arc::new(String::default()));

    // The ArcSwap type is a container for an Arc that can be changed atomically. Semantically, it is similar to something like Atomic<Arc<T>> (if there was such a thing) or RwLock<Arc<T>> (but without the need for the locking)
    thread::scope(|scope| {
        scope.spawn(|_| {
            let new_conf = Arc::new("New configuration".to_owned());
            config.store(new_conf);
        });
        for _ in 0..10 {
            scope.spawn(|_| loop {
                let cfg = config.load();
                if !cfg.is_empty() {
                    println!("{cfg}");
                    return;
                } else {
                    println!("cfg is empty");
                }
            });
        }
    })
    .unwrap();
}

#[cfg(test)]
mod tests {
    use arc_swap::ArcSwapOption;
    use std::sync::Arc;
    #[test]
    fn arc_swap_option_should_work() {
        let shared = ArcSwapOption::from(None);
        assert!(shared.load_full().is_none());
        assert!(shared.swap(Some(Arc::new(42))).is_none());
        assert_eq!(42, **shared.load().as_ref().unwrap());
    }
}
