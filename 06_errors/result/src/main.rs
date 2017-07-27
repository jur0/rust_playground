use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    // If Result is Ok, unwrap will return what's inside Ok otherwise it will
    // panic!
    //let f = File::open("hello.txt").unwrap();

    // The same as above, but we specify what panic! message is.
    // let f = File::open("hello.txt").expect("Cannot open the file");

   match f {
        Ok(file) => file,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            panic!("File not found");
        },
        Err(e) => {
            panic!("Error opening file: {:?}", e);
        },
    };
}
