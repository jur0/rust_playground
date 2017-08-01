
// LIFO queue
#[derive(Debug)]
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }
}

fn main() {
    {
        let mut cq = Queue::new();

        cq.push('a');
        cq.push('b');
        cq.push('c');

        println!("cq = {:?}", cq);

        let x = cq.pop();

        println!("cq = {:?}, x = {:?}", cq, x);
    }

    {
        // Explicit type definition.
        let mut sq = Queue::<String>::new();

        sq.push("abc".to_string());
        sq.push("def".to_string());

        println!("sq = {:?}", sq);

        let x = sq.pop();

        println!("sq = {:?}, x = {:?}", sq, x);
    }

    {
        // Rust can figure out the type itself from inserted values.
        let mut q1 = Queue::new();
        let mut q2 = Queue::new();

        // This is Queue<&'static str>
        q1.push("abc");
        // This is Queue<f64>
        q2.push("0.831");

        q1.push("xyz");
        q2.push("3.329");

        println!("q1 = {:?}", q1);
        println!("q2 = {:?}", q2);
    }
}
