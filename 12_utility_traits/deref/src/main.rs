use std::ops::{Deref, DerefMut};
use std::fmt::Display;

// Deref and DerefMut in standard library:
//trait Deref {
//    type Target: ?Sized;
//    fn deref(&self) -> &Self::Target;
//}
//
//trait DerefMut: Deref {
//    fn deref_mut(&mut self) -> &mut Self::Target;
//}

struct Selector<T> {
    // Elements available in selector.
    elements: Vec<T>,
    // The index of the current element.
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target=T;

    fn deref(&self) -> &T {
        println!("deref");
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        println!("deref_mut");
        &mut self.elements[self.current]
    }
}

fn main() {
    {
        let mut s = Selector { elements: vec!['x', 'y', 'z'], current: 2 };

        // Use * operator (deref).
        assert_eq!(*s, 'z');
        // Use method of char directly on Selector via deref coercion (deref).
        // *s returns &char and &char is converted to char.
        assert!(s.is_alphabetic());
        // Change an element by assigning to Selector (deref_mut).
        *s = 'w';
        assert_eq!(s.elements, ['x', 'y', 'w']);
    }

    {
        // Rust applies deref coercions to resolve type conflicts, but not to
        // satisfy bounds on type viariables.
        let s = Selector { elements: vec!["one", "two", "three"], current: 2 };

        fn show_it(element: &str) {
            println!("show_it: {}", element);
        }
        // All good. &Selector<&str> is converted to &str (there is
        // implementation for Deref<Target=str>, the call is rewritten to
        // show_it(s.deref())
        show_it(&s);

        fn show_it_again<T: Display>(element: T) {
            println!("show_it_again: {}", element);
        }
        // This doesn't work. We are passing &Selector<&str> and function's
        // param is of type &T, then type  variable T is Selector<&str>. Then
        // Rust checks if T: Display is satisfied, but it doesn't apply deref
        // coercions to satisfy bounds (so the check fails)!
        //show_it_again(&s);

        // Coercion must be spelled out.
        show_it_again(&s as &str);
    }
}
