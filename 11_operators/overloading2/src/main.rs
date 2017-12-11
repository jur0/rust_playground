use std::ops::{Add, AddAssign, Neg};

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    // Real portion of the complex number.
    r: T,
    // Imaginary portion of the complex number.
    i: T,
}

// This is the trait from std::ops::Add.
//trait Add<RHS=Self> {
//    type Output;
//
//    fn add(self, rhs: RHS) -> Self::Output;
//}

// T needs to implement the Add trait, and if we add a T to a T we
// get another T as the output. Since Add is part of the standard
// library, it is already implemented for all of the primitive
// integer and floating point types.
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

// T needs to implement the Neg trait, and if we neg a T, we get another
// T as the output.
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Complex {
            r: -self.r,
            i: -self.i,
        }
    }
}

// T need to implement the AddAssign trait, and if we += a T, we get
// another T which also implements the same trait.
impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

// For any T that itself needs to implement PartialEq, this implements
// comparison for Complex<T>.
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i
    }
}

fn main() {
    let c1 = Complex { r: 1.5, i: 2.5 };
    // This won't work. The complex must be the same type.
    //let c2 = Complex { r: 1, i: 2 };
    let c2 = Complex { r: 0.5, i: 0.5 };
    let c3 = c1 + c2;
    let mut c4 = Complex { r: 1, i: 1 };
    c4 += Complex { r: 2, i: 2 };

    let a = Complex { r: 0, i: 0 };
    let b = Complex { r: 0, i: 0 };

    println!("c3 = {:?}", c3);
    println!("-c3 = {:?}", -c3);
    println!("c4 = {:?}", c4);
    println!("{:?} == {:?} : {}", a, b, a == b);
}
