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

    impl<L, R, O> Add<Complex<R>> for Complex<L>
        where L: Add<R, Output=O>
    {
        type Output=Complex<O>;

        fn add(self, rhs: Complex<R>) -> Self::Output {
            Complex { r: self.r + rhs.r, i: self.i + rhs.i }
        }
    }

    // TODO: which 2 types does it allow to add together?
    let c1 = Complex { r: 10u32, i: 20u32 };
    let c2 = Complex { r: 5u16, i: 5u16 };
    let c3 = c1 + c2;

    println!("complex = {:?}", c3);
}
