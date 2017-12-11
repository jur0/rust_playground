fn main() {
    // Integers are simple values and are pushed onto stack, this will work.
    // It's copying on the stack.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // String is allocated on heap, so s1 and s2 are just pointers. There must
    // be always just 1 pointer to the same memory on the heap, so s1 will be
    // invalidated - it's called move (move s1 to s2 - s1 cannot be used
    // anymore).
    let s1 = String::from("hello");
    let s2 = s1;
    // this won't work!
    //println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // Cloning will copy the heap of z1 to z2 (z1 is still valid)
    let z1 = String::from("some data");
    let z2 = z1.clone();
    println!("z1 = {}, z2 = {}", z1, z2);

    // Here b1 is a reference to heap allocated f64. b1 is moved to b2, so b1
    // in no longer valid.
    let b1 = Box::new(0.55);
    let b2 = b1;
    println!("b2 = {}", b2);

    // Just as variables own their values, structs own their members, and
    // tuples, arrays and vectors own their elements.
    #[derive(Debug)]
    struct S {
        x: String,
        y: u32,
    };
    let s = S {
        x: "test string".to_string(),
        y: 100,
    };
    let xx = s.x;
    let yy = s.y;
    println!("xx = {}", xx);
    println!("yy = {}", yy);
    // this won't work, s.x was moved to xx.
    //println!("s = {:?}", s);

    #[derive(Debug)]
    let v = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
    // v is moved into for loop
    for mut s in v {
        s.push('!');
        println!("v = {}", s);
    }
    // this won't work as v was moved
    //println!("v = {:?}", v);

    {
        let s = String::from("hello");
        // Ownership of s is moved into the function and s is no longer valid
        // here.
        take_ownership(s);

        let x = 5;
        // x is i32 which is Copy, so it's fine to use x afterwards.
        make_copy(x);
    } // Here, x goes out of scope, then s. But since s's value was moved,
      // nothing special happens.

    {
        // give_ownership moves its return value to s1
        let s1 = give_ownership();
        let s2 = String::from("hello");
        // s2 is moved into take_and_give_ownership_back, which also moves its
        // return value into s3.
        let s3 = take_and_give_ownership_back(s2);
        // s2 won't work in println!
        println!("s1 = {}, s3 = {}", s1, s3);
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but
      // was moved, so nothing happens. s1 goes out of scope and is dropped.
}

fn take_ownership(s: String) {
    println!("take_ownership: s = {}", s);
} // s goes out of scope and `drop` is called. The backing memory is freed.

fn make_copy(i: i32) {
    println!("make_copy: i = {}", i);
} // i goes out of scope. Nothing special happens.

fn give_ownership() -> String {
    let s = String::from("hello");
    s
}

fn take_and_give_ownership_back(s: String) -> String {
    s
}
