use std::fmt::Debug;

// T1 must be Debug to satisfy the say_hello function.
trait T1: Debug {
    fn description(&self) -> String;
    fn double(&mut self);
    // This is a default implementation.
    fn say_hello(&self) {
        println!("hello from: {:?}", self);
    }
}

#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct B {
    x: i32,
    y: i32,
    z: i32,
}

impl T1 for A {
    fn description(&self) -> String {
        format!("struct A {{ x = {}, y = {} }}", self.x, self.y)
    }

    fn double(&mut self) {
        self.x *= 2;
        self.y *= 2;
    }

    // Rewriting the default implementation.
    fn say_hello(&self) {
        println!("HOLA from: {:?}", self);
    }
}

impl T1 for B {
    fn description(&self) -> String {
        format!(
            "struct A {{ x = {}, y = {}, z = {} }}",
            self.x, self.y, self.z
        )
    }

    fn double(&mut self) {
        self.x *= 2;
        self.y *= 2;
        self.z *= 2;
    }
}

// This is a plain function taking a trait object as an argument.
fn double(x: &mut T1) {
    x.double();
}

// The above function can be rewritten to a generic form.
fn gen_double<T: T1>(x: &mut T) {
    x.double();
}

fn main() {
    {
        let a = A { x: 2, y: 4 };
        let mut b = B { x: 3, y: 6, z: 9 };

        b.double();

        println!("description a = {}", a.description());
        println!("description b = {}", b.description());
    }

    {
        let mut a = A { x: 10, y: 20 };
        // This won't work. T1 is a trait, and its referent type is unknown
        // at compile time hence "T1 doesn't have a constant size at compile
        // time" error.
        //let t1: T1 = a;

        println!("before double: {}", a.description());
        // Reference is converted to a trait object.
        double(&mut a);
        // Reference to a trait is trait object.
        let t1: &mut T1 = &mut a;
        println!("after double: {}", t1.description());

        // Box can become a fat pointer - trait object.
        // Box<B> is converted to Box<T1>
        let t2: Box<T1> = Box::new(B {
            x: 100,
            y: 200,
            z: 300,
        });
        // TODO: Box pointing to a mutable memory?
        //t2.double();
        println!("{}", t2.description());
    }

    {
        let mut a = A { x: 2, y: 4 };
        let mut b = B { x: 3, y: 6, z: 9 };

        // Code is generated for A and B types.
        gen_double(&mut a);
        gen_double(&mut b);

        println!("description a = {}", a.description());
        println!("description b = {}", b.description());

        a.say_hello();
        b.say_hello();
    }
}
