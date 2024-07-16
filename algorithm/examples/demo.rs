use std::{cell::RefCell, rc::Rc};

fn main() {
    let i = 100;
    let mut i_box = Box::new(i);
    println!("{i_box}");
    *i_box = 200;
    println!("{i_box}");

    let i_rc = Rc::new(i);
    println!("{i_rc}");

    let i_rc_refcell = Rc::new(RefCell::new(i));
    println!("{}", i_rc_refcell.borrow());
    *i_rc_refcell.borrow_mut() = 200;
    println!("{}", i_rc_refcell.borrow());
}
