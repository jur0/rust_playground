// Clone trait extends Sized. The types it works with are Sized.

#[derive(Debug)]
struct A {
    x: String,
}

// Clone from the standard library:
//trait Clone : Sized {
//    fn clone(&self) -> Self;
//    fn clone_from(&mut self, source: &Self) {
//        *self = source.clone()
//    }
//}

impl Clone for A {
    fn clone(&self) -> Self {
        println!("clone");
        A { x: self.x.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        println!("clone from");
        self.x = source.x.clone();
    }
}

fn main() {
    let x = A {
        x: "struct_x".to_string(),
    };
    let xx = x.clone();

    println!("xx = {:?}", xx);

    let y = A {
        x: "struct_y".to_string(),
    };
    let mut yy = A {
        x: "just struct".to_string(),
    };
    yy.clone_from(&y);

    println!("yy = {:?}", yy);
}
