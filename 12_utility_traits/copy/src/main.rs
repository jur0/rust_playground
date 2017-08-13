
// Copy is shallow byte to byte copy.

#[derive(Debug, Copy)]
struct A {
    x: i32
}

//#[derive(Debug, Copy)]
// This doesn't work because String doesn't implement Copy.
//struct B {
//    x: String
//}

fn main() {
    let a = A { x: 10 };
    let aa = a;
    println!("aa = {:?}", aa);

    //let b = B { x: "aaa".to_string() };
    //let bb = b;
    //println!("bb = {:?}", bb);
}
