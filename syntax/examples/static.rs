fn main() {
    let msg = "hello";
    info(msg);

    // let msg = "hello".to_string();
    // info(&msg);

    static MSG: &str = "world";
    info(MSG);
}

fn info(msg: &'static str) {
    println!("{msg}");
}
