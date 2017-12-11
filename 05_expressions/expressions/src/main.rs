fn main() {
    {
        // This is declaration, not expression. The let declaration doesn't
        // have to initialise the variable. The variable must be initialised
        // later.
        let var;
        // This is considered initialisation. Note that var is immutable.
        var = 10;
        // Error: assignement to immutable variable.
        //var = 20;

        assert!(var == 10);
    }

    {
        // Blocks are expressions.
        let a = { 10 }; // Returns 10.
        let b = {
            10;
        }; // Returns () - due to semicolon.

        assert_eq!(a, 10);
        assert_eq!(b, ());
    }

    {
        let x = Some(100);
        let y = None;

        fn check_value(val: &Option<i32>) {
            // "if let" expression.
            if let &Some(_) = val {
                println!("there is something");
            } else {
                println!("there is nothing");
            }
        }

        check_value(&x);
        check_value(&y);
    }

    // Loops

    {
        let mut i = 0;
        let last = 20;

        println!("loop: ");
        loop {
            if i < last - 1 {
                print!("{} ", i);
                i += 1;
            } else {
                println!("{}", i);
                break;
            }
        }
    }

    {
        let mut i = 0;
        let last = 20;

        println!("while: ");
        while i < last {
            if i == (last - 1) {
                println!("{}", i);
            } else {
                print!("{} ", i);
            }
            i += 1;
        }
    }

    {
        let last = 20;

        println!("for: ");
        for i in 0..last {
            if i == last - 1 {
                println!("{}", i);
            } else {
                print!("{} ", i);
            }
        }
    }

    {
        // TODO: while let pattern = expr
        // TODO: unwrap, expect, ?, return
    }
}
