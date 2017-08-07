
fn main() {
    {
        // collect is a generic method. In this case we need to specify
        // the type parameter.
        let v = (0 .. 10).collect::<Vec<i32>>();
        println!("v = {:?}", v);
    }

    {
        use std::fmt::{Debug, Display};

        fn compare_prints<T: Debug + Display>(t: &T) {
            println!("Debug: `{:?}`", t);
            println!("Display: `{}`", t);
        }

        // This is the same as the previous function. It just shows the use
        // of where keyword.
        fn compare_prints2<T>(t: &T)
            where T: Debug + Display
        {
            println!("Debug: `{:?}`", t);
            println!("Display: `{}`", t);
        }

        fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
            println!("t: {:?}", t);
            println!("u: {:?}", u);
        }

        let string = "words";
        let array = [1, 2, 3];
        let vec = vec![1, 2, 3];

        compare_prints(&string);
        compare_prints2(&string);
        // This won's work. Display is not implemented for array.
        //compare_prints(&array);

        compare_types(&array, &vec);
    }

    {
        trait Parity {
            fn is_even(&self) -> bool;

            fn is_odd(&self) -> bool {
                !self.is_even()
            }
        }

        // Parity is extension trait as it extends the basic type i32.
        impl Parity for i32 {
            fn is_even(&self) -> bool {
                self % 2 == 0
            }
        }

        println!("is even 10: {}", 10.is_even());
        println!("is odd 20: {}", 20.is_odd());
    }

    {
        // Subtrait - every type that implemets A must also implement B.
        trait A: B {
            fn a_function(&self) {
                println!("A trait");
            }
        }

        trait B {
            fn b_function(&self) {
                println!("B trait");
            }
        }

        impl A for bool { }
        impl B for bool { }

        true.a_function();
        false.b_function();
    }

    {
        trait StringSet {
            // The first 2 functions are static and won't work with trait objects.
            // Using the Sized bound the trait objects are excused for supporting
            // those methods.
            fn new() -> Self
                where Self: Sized;

            fn from_slice(strings: &[&str]) -> Self
                where Self: Sized;

            fn contains(&self, string: &str) -> bool;

            fn add(&mut self, string: &str);
        }

        use std::collections::HashSet;

        impl StringSet for HashSet<String> {
            fn new() -> Self {
                HashSet::new()
            }

            fn from_slice(strings: &[&str]) -> Self {
                let mut set = HashSet::new();

                for string in strings {
                    set.insert(string.to_string());
                }
                set
            }

            fn contains(&self, string: &str) -> bool {
                self.contains(&string.to_string())
            }

            fn add(&mut self, string: &str) {
                self.insert(string.to_string());
            }
        }

        // Generic function.
        fn unknown_words<S: StringSet>(document: &Vec<&str>, wordlist: &S) -> S {
            let mut unknowns = S::new();
            for word in document {
                if !wordlist.contains(word) {
                    unknowns.add(word);
                }
            }
            unknowns
        }

        // Trait object.
        fn is_word_present(unknowns: &StringSet, word: &str) -> bool {
            unknowns.contains(word)
        }

        let doc = vec!["one", "one", "two", "three", "four", "four", "five", "six"];
        let wlist = HashSet::<String>::new();

        let unknowns = unknown_words(&doc, &wlist);
        println!("unknowns = {:?}", unknowns);

        let x = "one";
        println!("{} in unknowns? {}", x, is_word_present(&unknowns, x));
    }

    {
        "hello".to_string();
        // Qualified call - static method call.
        str::to_string("hello");
        // Qualified call - ToString is a trait.
        ToString::to_string("hello");
        // Fully qualified method call.
        <str as ToString>::to_string("hello");

        // All four calls do the same thing.
    }
}
