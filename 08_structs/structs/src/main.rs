
// Named-field structs.
#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
}

// Tuple-like structs.
#[derive(Debug)]
struct B(i32, i32);

// Unit-like structs.
#[derive(Debug)]
struct C;

fn main() {
    let a = A { x: 1, y: 2};
    let b = B(1, 2);
    let c = C;

    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);
}
