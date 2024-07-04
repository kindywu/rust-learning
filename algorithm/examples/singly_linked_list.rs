use std::{cell::RefCell, rc::Rc};

struct Node {
    v: i32,
    next: Option<Rc<RefCell<Node>>>,
}

// #[allow(clippy::assigning_clones)]
fn main() {
    let head = Rc::new(RefCell::new(Node { v: 0, next: None }));
    let mut current = head.clone();
    for i in 1..10 {
        let new_node = Rc::new(RefCell::new(Node { v: i, next: None }));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
    }

    current = head.clone();
    // let mut current = head;
    println!("loop");
    loop {
        print!("{:5}", current.borrow().v);
        let Some(tmp) = current.borrow().next.clone() else {
            break;
        };
        current = tmp;
    }
    println!();

    let mut current = Some(head);
    println!("while");
    while let Some(node) = current {
        let node = node.borrow();
        print!("{:5}", node.v);
        if let Some(next_node) = &node.next {
            current = Some(next_node.clone());
        } else {
            current = None;
        }
    }
}
