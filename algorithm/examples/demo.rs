fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);

    println!("pop {:?}", stack.pop());
    println!("pop {:?}", stack.pop());
}
