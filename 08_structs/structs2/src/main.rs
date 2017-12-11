// LIFO queue
#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

// impl doesn't have pub keyword associated with it.
impl Queue {
    // Associated function (doesn't have self parameter).
    fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    // Push a char onto the back of a queue.
    fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    // Pop a char off the front of a queue.
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Move from younger to older.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // older is guaranteed to have some elements.
        self.older.pop()
    }
}

fn main() {
    let mut q = Queue::new();

    q.push('a');
    q.push('b');
    q.push('c');

    println!("q = {:?}", q);

    let x = q.pop();

    println!("q = {:?}, x = {:?}", q, x);
}
