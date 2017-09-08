
struct I32Range {
    start: i32, // the next value
    end: i32,   // limit
}

impl Iterator for I32Range {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

// The standard library provides a blanket implementation of IntoIterator for
// every type that implements Iterator.

fn main() {
    let mut pi = 0.0;
    let mut numerator = 1.0;

    // for loop uses .into_iter()
    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }

    pi *= f64::sqrt(12.0);

    assert_eq!(pi as f32, std::f32::consts::PI);
}
