fn main() {
    {
        let x = 4;
        // This won't work - functions can't capture environment.
        //fn equal_to_x(z: i32) -> bool { z == x }

        // Closures can capture the environment.
        // x is Copy so it's copied into the closure.
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
    }

    {
        let x = vec![1, 2, 3];

        // If we want to force the closure to take ownership of the values it
        // uses in the environment, we can use the move keyword before the
        // parameter list. The move keyword is added to the closure definition
        // and using vectors instead of integers, since integers can be copied
        // rather than moved.
        let equal_to_x = move |z| z == x;
        //println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }
}
