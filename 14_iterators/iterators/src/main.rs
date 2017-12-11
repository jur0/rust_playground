// Iterator trait from standard library:
//trait Iterator {
//    type Item;
//    fn next(&mut self) -> Option<Self::Item>;
//    ... // many default methods
//}

// IntoIterator trait from standard librabry:
//trait IntoIterator
//    where Self::IntoIter::Item == Self::Item {
//
//    type Item;
//    type IntoIter: Iterator;
//    fn into_iter(self) -> Self::IntoIter;
//}
// If there is a natural way to iterate over a type, it implements the above
// trait.

fn main() {
    {
        let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

        // Without for loop (for loop gets translated into this):
        let mut iterator = (&v).into_iter();
        while let Some(element) = iterator.next() {
            println!("{}", element);
        }

        println!("-----");

        // With for loop:
        for element in &v {
            println!("{}", element);
        }
    }

    {
        println!("-----");

        let mut v = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];

        // (&v).into_iter()
        for x in &v {
            println!("{}", *x);
        }

        println!("-----");

        // (&mut v).into_iter()
        for x in &mut v {
            *x += "append";
            println!("{}", *x);
        }

        println!("-----");

        // v.into_iter() - consumes the vector.
        for x in v {
            println!("{}", x);
        }

        // v was consumed before, cannot be used again.
        //for x in v {
        //    println!("{}", x);
        //}
    }

    {
        let v = vec![4, 20, 12, 8, 6];
        // v.iter() returns iterator over references in the vector.
        // v.into_iter() would return iterator over values and consume the
        // vector.
        let mut iterator = v.iter();

        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&12));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    {
        let text = " ponies \n giraffes\niguanas \nsquid".to_string();

        // text.lines() returns iterator.
        let v: Vec<&str> = text.lines()
            // Calling map on iterator returns second iterator that applies
            // str::trim on each item.
            .map(str::trim)
            // Calling filter returns third iterator.
            .filter(|&s| s != "iguanas")
            // This one also works
            //.filter(|s| s != &"iguanas")
            // Calling collet gathers items into vector.
            .collect();

        assert_eq!(v, ["ponies", "giraffes", "squid"]);
    }
}
