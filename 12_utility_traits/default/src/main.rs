// Default trait from standard librabry:
//trait Default {
//    fn default() -> Self;
//}

// Rust doesn't implement Default for structs. It can be added using
// #[derive(Default)].
#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
}

impl Default for A {
    fn default() -> Self {
        A { x: 1, y: -1 }
    }
}

fn main() {
    let a = A::default();
    println!("a = {:?}", a);
}
