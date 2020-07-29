use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    //Rc allows multiple owners, we wrap the RefCell that allows mut over ref
    // the second element must be part of the List enum that can be owned by
    // many as well
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

fn main() {
    // we generate a multiple owned ref to the mutable ref that holds the value
    // 5
    let value = Rc::new(RefCell::new(5));
    // we generate a List that clones the ref of 5, allocate the Nil value as
    // a ref as well and make the whole chunk of the list a ref that can be
    // owned
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // we clone the ref on b and c, but we dont make this chunks multi ownable
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    // we modify the value of the reference inside value
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
// similar structures Cell<T>[works similar but instead of giving
// a ref it gives a copy] and Mutex<T>

