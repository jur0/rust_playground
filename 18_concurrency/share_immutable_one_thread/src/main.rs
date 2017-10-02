
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // This won't work with Box pointer.
    //Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // a is moved here.
    //let b = Cons(3, Box::new(a));
    // a is used again after the move.
    //let c = Cons(4, Box::new(a));

    // a is shared with b and c
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, a.clone());
    let c = Cons(4, a.clone());

    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
