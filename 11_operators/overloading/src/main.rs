use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    // Real portion of the complex number.
    r: T,
    // Imaginary portion of the complex number.
    i: T
}

// This is the trait from std::ops::Add.
//trait Add<RHS=Self> {
//    type Output;
//
//    fn add(self, rhs: RHS) -> Self::Output;
//}

fn main() {
    // Non-generic implementation of the trait.
    // This it the same as (because RHS=Self, so Complex<i32> is Self):
    //impl<Complex<i32>> for Complex<i32> { ... }
    //           |---Self---|
    impl Add for Complex<i32> {
        type Output=Self;

        fn add(self, rhs: Self) -> Self {
            Complex { r: self.r + rhs.r, i: self.i + rhs.i }
        }
    }

    let c1 = Complex { r: 10, i: 20 };
    let c2 = Complex { r: 5, i: 5 };
    let c3 = c1 + c2;

    println!("complex = {:?}", c3);
}
