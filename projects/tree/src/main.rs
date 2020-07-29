use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    // we have a normal rc cause if the parent drops the chidlren should
    // perish too but a weak on the parent cause if the child dies we dont
    // kill the parent node. a child when instancaited has a strong ref to
    // the parent which increments teh strong count if the parent dies the
    // count should go down and if no one else points to it it should drop to
    // 0 and be gone, not the same with the parent scenario
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}


fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    println!(
        "leaf => [wcount={}, scount={}]",
        Rc::weak_count(&leaf),
        // 1 for strong count as leaf owns itself
        Rc::strong_count(&leaf)
    );
    // we use a inner scope to see how branch goes out of scope
    {
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        println!(
            "leaf => [wcount={}, scount={}]",
            Rc::weak_count(&leaf),
            Rc::strong_count(&leaf)
        );
        println!(
            "branch => [wcount={}, scount={}]",
            Rc::weak_count(&branch),
            Rc::strong_count(&branch)
        );
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "leaf => [wcount={}, scount={}]",
            Rc::weak_count(&leaf),
            Rc::strong_count(&leaf)
        );
        println!(
            "branch => [wcount={}, scount={}]",
            Rc::weak_count(&branch),
            Rc::strong_count(&branch)
        );
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    // branch no longer exists and leaft goes back to having an scount of 1
    println!(
        "leaf => [wcount={}, scount={}]",
        Rc::weak_count(&leaf),
        Rc::strong_count(&leaf)
    );
    // notice this does not produce a memory leak loop that floods the stack
    // as the strong count is not incremented by the weak reference to the 
    // parent they go out of scope safely.
}
