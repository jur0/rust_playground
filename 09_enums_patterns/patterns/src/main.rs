
fn main() {
    {
        let y = 'y';

        // ref on the left side of an assignment is like adding `&` on the
        // right side.
        let ref x1 = y;
        let x2 = &y;

        println!("{}", x1 == x2);
    }

    {
        let a = 10;

        match Some(a) {
            // aa = &a
            Some(ref aa) => println!("[a -> ref aa] *aa = {}, aa = {}", *aa, aa),
            None => println!("nothing"),
        }
        println!("a = {}", a);

        match Some(&a) {
            // aa = &a
            Some(aa) => println!("[&a -> aa] *aa = {}, aa = {}", *aa, aa),
            None => println!("nothing"),
        }
        println!("a = {}", a);

        match Some(&a) {
            // aa = a - it's destructured the same way it's constructed.
            Some(&aa) => println!("[&a -> &aa] aa = {}", aa),
            None => println!("nothing"),
        }
        println!("a = {}", a);
    }

    {
        let s = String::from("hello");

        match Some(s) {
            // ss = &s
            Some(ref ss) => println!("[s -> ref ss] *ss = {}, ss = {}", *ss, ss),
            None => println!("nothing"),
        }
        //println!("s = {}", s); - this won't work, s was moved in 'match Some(s)'

        let s = String::from("hello");
        match Some(&s) {
            Some(ss) => println!("[&s -> ss] *ss = {}, ss = {}", *ss, ss),
            None => println!("nothing"),
        }
        println!("s = {}", s);

        match Some(&s) {
            // ss = s - this won't work! "cannot move out of borrowed content"
            // &s is a reference to s - borrowing. Some(&ss) tries to convert
            // ss to s, which would change the ownership. We cannot change
            // ownership of something we borrowed.
            //Some(&ss) => println!("[&s -> &ss] ss = {}", ss),

            // ref ss -> &s -> s -> "hello"
            Some(ref ss) => println!("[&s -> ref ss] **ss = {}, *ss = {}, ss = {}", **ss, *ss, ss),
            None => println!("nothing"),
        }
        println!("s = {}", s);
    }

    {
        #[derive(Clone, Copy, Debug)]
        struct X {
            x: u32,
        };

        #[derive(Debug)]
        struct Y {
            y: u32,
        };

        let x = X { x: 100 };
        let y = Y { y: 100 };

        match Some(&x) {
            Some(&xx) => println!("(copy) [&x -> &xx] xx = {:?}", xx),
            None => println!("nothing"),
        }

        match Some(&y) {
            // This won't work!
            //Some(&yy) => println!("(clone) [&y -> &yy] yy = {:?}", yy),

            Some(ref yy) => println!("(clone) [&y -> ref yy] **yy = {:?}, *yy = {:?}, yy = {:?}",
                                     **yy, *yy, yy),
            None => println!("nothing"),
        }
    }

    {
        // This enum needs 1 byte for tag, and a pointer to vector on heap,
        // and size with capacity. Also None takes up the same amount of
        // memory.
        #[derive(Debug)]
        #[allow(dead_code)] // None unused.
        enum X<T> {
            Some(T),
            None,
        }
        let x = X::Some(vec![1, 2, 3]);
        println!("x = {:?}", x);

        // This enum stores just Box pointer (in case of Some) or 0 in case
        // of None.
        #[derive(Debug)]
        #[allow(dead_code)] // None unused.
        enum Y<T> {
            Some(Box<T>),
            None,
        }

        let y = Y::Some(Box::new(vec![1, 2, 3]));
        println!("y = {:?}", y);
    }
}
