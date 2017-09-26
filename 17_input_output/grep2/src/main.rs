use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn grep<R>(target: &str, reader: R) -> io::Result<()>
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    // OK
    //let stdin = io::stdin();
    //grep(&target, stdin.lock())?;

    let f = File::open("test.txt").unwrap();
    grep("abc", BufReader::new(f)).unwrap();
}
