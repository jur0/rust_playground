use std::fmt::Display;

// str is unsized type (without &).
// [T] is unsized as well (without &).
// Referent of a trait object is unsized.
// Struct's last field may be unsized and so the whole struct is unsized.

// T is Size by default.
#[derive(Debug)]
struct A<T> {
    x: i32,
    y: T,
}

// ?Sized means "not necessarily Sized".
struct B<T: ?Sized> {
    #[allow(dead_code)] x: i32,
    y: T,
}

// All the generic type parameters are Sized implicitly.
fn main() {
    let a: A<i32> = A { x: 10, y: 20 };
    println!("Sized a: {:?}", a);

    // Value of unsized struct B can't be created directly, instead, a sized
    // B struct must be created at first. Then a reference to the created
    // sized value can be converted to a fat reference.
    // String is sized.
    let b: B<String> = B {
        x: 100,
        y: "hello".to_string(),
    };
    let bb: &B<Display> = &b;

    // TODO: this doesn't work
    //println!("bb.y = {}", bb.y);

    fn display(d: &B<Display>) {
        println!("?Sized display: d.y = {}", &d.y)
    }

    // The reference &B<String> is converted to fat reference &B<Display>.
    display(&b);
    display(bb);
}
