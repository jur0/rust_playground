
fn main() {
    {
        let x = 10;
        let y = 20;
        let mut p = &x;

        println!("p = {}", p);
        {
            p = &y;
        }
        println!("p = {}", p);
    }

    {
        #[derive(Debug)]
        struct Point { x: i32, y: i32 }
        let point = Point { x: 10, y: 20 };

        let r: &Point = &point;
        // References to references.
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;

        // Automatic dereference.
        assert_eq!(rrr.x, 10);
        assert_eq!(rrr.y, 20);

        assert_eq!((***rrr).x, 10);
        assert_eq!((***rrr).y, 20);

        println!("***rrr = {:?}", ***rrr);
    }

    {
        let x = 0;
        let y = 0;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        // Automatic dereference.
        assert!(rx <= ry);
        assert!(rx == ry);

        // Aautomatic dereference.
        assert!(rrx <= rry);
        assert!(rrx == rry);

        assert!(*rx <= *ry);
        assert!(*rx == *ry);

        assert!(**rrx <= **rry);
        assert!(**rrx == **rry);

        // std::ptr::eq compares references as addresses.
        assert!(!std::ptr::eq(rrx, rry));
        assert!(std::ptr::eq(rx, &x));
    }

    {
        fn factorial(n: usize) -> usize {
            (1..n + 1).fold(1, |a, b| a * b)
        }

        // Rust allows to borrow a reference to the value of any expression,
        // such as function. Rust creates an anonymous variable to hold the
        // expression's (function's) value. r_fact is a reference that points
        // to that.
        let r_fact = &factorial(6);
        println!("r_fact = {}", r_fact);

        // 1009 lasts only to the end of assert_eq! statement.
        assert_eq!(r_fact + &1000, 1720);
        assert_eq!(r_fact+ 1000, 1720);
        assert_eq!(*r_fact + 1000, 1720);
    }

    {
        static mut STASH: &i32 = &10;
        static VALUE: i32 = 123;

        // Function signature must use 'static lifetime here, otherwise it
        // won't compile. 'static lifetime is from the beginning of execution
        // till the end.
        fn f(p: &'static i32) {
            // Static mut must always be in unsafe block.
            unsafe {
                STASH = p;
            }
        }

        f(&VALUE);
        unsafe {
            println!("STASH = {}", STASH);
        }
    }

    {
        // Function print1 is equivalent of print2. The 'a lifetime is added
        // to print1 by default, so it doesn't have to be written explicitely.
        fn print1(p: &i32) {
            println!("print1: p = {}", p);
        }

        fn print2<'a>(p: &'a i32) {
            println!("print2: p = {}", p);
        }

        let x = 100;
        print1(&x);
        print2(&x);
    }

    {
        // This function is equivalent to: fn smallest<'a>(v: &'a [i32]) -> &'a i32
        // v must have at least 1 element.
        fn smallest(v: &[i32]) -> &i32 {
            let mut s = &v[0];
            for r in &v[1..] {
                if *r < *s {
                    s = r;
                }
            }
            s
        }

        let arr = [5, 3, 8, 2, 1];
        let smallest = smallest(&arr);
        println!("smallest = {}", smallest);
    }

    {
        // Whenever there is a refence in another's type definition, the
        // reference must have its lifetime specified.
        #[derive(Debug)]
        struct S<'a> {
            r: &'a i32,
        };

        let x = 10;
        // When &x is stored in r, 'a must lie entirely in x's lifetime.
        // x must live longer than s.
        let s = S { r: &x };
        println!("s = {:?}", s);
    }

    {
        #[derive(Debug)]
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32,
        };

        let x = 10;
        let rx;
        //let ry;
        {
            let y = 20;
            {
                // &x must live longer than s.x.
                // &y must live longer than s.y.
                let s = S { x: &x, y: &y };
                // s.x (&x) must live longer than r.
                rx = s.x;
                // This won't work: s.y (&y) must live longer than ry.
                //ry = s.y;
            }
        }
        println!("rx = {}", rx);
    }
}
