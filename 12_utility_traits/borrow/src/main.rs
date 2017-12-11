use std::borrow::Borrow;
use std::fmt::Display;

// This trait is similar to AsRef. However, Borrow is more restrictive:
// a type can implement Borrow<T> when &T hashes and compares the same way
// as the value borrowed from (i.e. &str hashes to the same value as String).

// Definition from standard library:
//trait Borrow<Borrowed: ?Sized> {
//    fn borrow(&self) -> &Borrowed;
//}

fn foo<T>(t: T)
where
    T: Borrow<i32> + Display,
{
    println!("t is borrowed: {}", t);
}

fn main() {
    let mut i = 5;

    // Even if foo takes T (not &T), &T works (as well as &mut T).
    foo(&i);
    foo(&mut i);
}
