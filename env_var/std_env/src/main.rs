use std::env;

fn main() {
    // List of all enviromental variables of this process.
    for (key, value) in env::vars_os() {
        println!("key: {:?}, value: {:?}", key, value);
    }
}
