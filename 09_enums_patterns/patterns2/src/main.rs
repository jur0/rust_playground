fn main() {
    {
        let x = 3;

        let result = match x {
            1 => "one",
            2 => "two",
            // Catch-all.
            _ => "other",
        };
        println!("result = {}", result);
    }

    {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 5, y: 8 };

        match p {
            Point { x: 5, y: 10 } => {
                println!("point: Point {{ x = 5, y = 10 }}");
            }
            Point { x, y } => {
                println!("catch-all point: Point {{ x = {}, y = {} }}", x, y);
            }
        }
    }

    // TODO: match multipe, ranges, guards, @ patterns, if let, let while
}
