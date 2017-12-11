use std::ops::Drop;

#[derive(Debug)]
struct A {
    x: String,
    y: Vec<String>,
}

// The standard drop function (takes ownership of _x, which just goes out of
// scope):
//fn drop<T>(_x: T) { }

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping struct A: {:?}", self);
    }
}

fn main() {
    let mut a = A {
        x: "aaa".to_string(),
        y: vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
    };

    println!("before assignment");

    // a is dropped after the assignment.
    a = A {
        x: "bbb".to_string(),
        y: vec![],
    };

    println!("end of block");
    // a is droppped again, after the block end.
}
