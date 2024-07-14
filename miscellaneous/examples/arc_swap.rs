use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_utils::thread;

fn main(){
    let config = ArcSwap::from(Arc::new(String::default()));
    thread::scope(|scope| {
        scope.spawn(|_| {
            let new_conf = Arc::new("New configuration".to_owned());
            config.store(new_conf);
        });
        for _ in 0..10 {
            scope.spawn(|_| {
                loop {
                    let cfg = config.load();
                    if !cfg.is_empty() {
                        println!("{cfg}");
                        return;
                    }else{
                        println!("cfg is empty");
                    }
                }
            });
        }
    }).unwrap();


}