use std::{rc::Rc, sync::Arc};
use tokio::{runtime::Runtime, task::LocalSet};

fn main() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        // do not need `Send+'static`
        let local = LocalSet::new();
        let rc = Rc::new(42);
        local
            .run_until(async move {
                println!("rc={rc}");
            })
            .await;

        // need `Send+'static`
        let arc = Arc::new(42);
        tokio::spawn(async move {
            println!("arc={arc}");
        })
    });
}
