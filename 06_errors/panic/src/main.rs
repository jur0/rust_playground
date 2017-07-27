
fn main() {
    // Macro panic! is used for crashing explicitly.
    //panic!("Something bad happened");

    let v = vec!['a', 'b', 'c'];
    // Trying to access array element which doesn't exist will cause Rust to
    // panic.
    let x = v[100];
    println!("x = {}", x);
}
