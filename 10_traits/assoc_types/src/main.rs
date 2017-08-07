
fn main() {
    // Example without associated types in a trait.
    {
        struct Container(i32, i32);

        // Trait which checks if 2 items are stored inside of container.
        // Also retrieves first or last value.
        trait Contains<A, B> {
            // Explicitly requires A and B.
            fn contains(&self, &A, &B) -> bool;
            fn first(&self) -> A;
            fn last(&self) -> B;
        }

        impl Contains<i32, i32> for Container {
            // True if the numbers stored are equal.
            fn contains(&self, n1: &i32, n2: &i32) -> bool {
                (&self.0 == n1) && (&self.1 == n2)
            }

            fn first(&self) -> i32 { self.0 }

            fn last(&self) -> i32 { self.1 }
        }

        fn difference<C>(container: &C) -> i32
            where C: Contains<i32, i32> {
                container.last() - container.first()
            }

        let n1 = 3;
        let n2 = 10;
        let container = Container(n1, n2);

        println!("Does container contain {} and {}: {}",
                 n1, n2, container.contains(&n1, &n2));
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }

    // The same as above, but with associated types in trait.
    {

        struct Container<T>(T, T);

        trait Contains {
            // There could have been just one associated type. There are two
            // to show more types are possible.
            type A;
            type B;

            fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool;
            fn first(&self) -> Self::A;
            fn last(&self) -> Self::B;
        }

        impl Contains for Container<i32> {
            type A = i32;
            type B = i32;

            fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool {
                (self.0 == *n1) && (self.1 == *n2)
            }

            fn first(&self) -> Self::A { self.0 }

            fn last(&self) -> Self::B { self.1 }
        }

        //use std::ops::Sub;
        //fn difference<T>(container: &T) -> T::A ??
        //    where T: Contains,
        //          T::A: Sub,
        //          T::B: Sub {
        //        container.last() - container.first()
        //}

        let n1 = 3;
        let n2 = 10;
        let container = Container(n1, n2);

        println!("Does container contain {} and {}: {}",
                 n1, n2, container.contains(&n1, &n2));
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        //println!("The difference is: {}", difference(&container));
    }

    {
        /*
        trait Mul<RHS=Self> { }

        impl Mul for Complex { }
        impl Mul<i32> for i32 { }

        // where T: Mul == where T: Mul<T>
        */
    }
}
