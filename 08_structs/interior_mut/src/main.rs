use std::cell::Cell;
use std::cell::RefCell;

#[derive(Debug)]
struct A {
    a: Cell<i32>,
    b: i32,
}

#[derive(Debug)]
struct B {
    a: RefCell<Vec<i32>>,
    b: i32,
}

impl A {
    fn new(a: i32, b: i32) -> Self {
        Self {
            a: Cell::new(a),
            b: b,
        }
    }

    // Setting value of the Cell via immutable reference.
    fn set_a(&self, v: i32) {
        self.a.set(v);
    }
}

impl B {
    fn new(a: Vec<i32>, b: i32) -> Self {
        Self {
            a: RefCell::new(a),
            b: b,
        }
    }

    // Setting value of the RefCell via immutable reference.
    fn add_to_a(&self, v: i32) {
        let mut vec = self.a.borrow_mut();
        vec.push(v);
    }
}

fn main() {
    {
        // Cell works only for Copy types.
        let x = Cell::new(1);
        let y = &x;
        let z = &x;

        x.set(2);
        // Setting values via immutable references. This normally doesn't work
        // without Cell.
        y.set(3);
        z.set(4);

        println!("x = {}", x.get());
    }

    {
        let x = A::new(1, 1);
        x.set_a(2);

        println!("x = {:?}", x);
    }

    {
        let x = B::new(vec![1, 2, 3], 1);
        x.add_to_a(4);

        println!("x = {:?}", x);
    }
}
