use std::future::Future;

fn main() {
    assert_unpin(1);
    assert_unpin("a");
    assert_unpin(vec![1, 2, 3]);

    // let f = async { println!("async") };
    // assert_unpin(f);

    // async fn f() {
    //     println!("async fn")
    // }
    // assert_unpin(f());

    assert_unpin(User::new());

    // runtime(f());
    runtime(Box::pin(f()));
}

fn assert_unpin<T: Unpin>(_t: T) {}

async fn f() {
    println!("async fn")
}

struct User {
    _name: String,
    // _t: std::marker::PhantomPinned,
}

impl User {
    fn new() -> User {
        Self {
            _name: "".to_string(),
        }
    }
}

// impl !Unpin for User {}

fn runtime<F>(_f: F)
where
    F: Future + Unpin,
{
}
