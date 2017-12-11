extern crate rust_lib;

use rust_lib::rust_mod::A;

fn main() {
    let mut x = A::new();
    x.set(1, 2);

    println!("x = {:?}", x);
}
