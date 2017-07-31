
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("test");
            }
        }
    }
}

#[derive(Debug)]
enum TrafficLights {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
enum Numbers {
    One,
    Two,
    Three,
}

use a::series::of::nested_modules;
use TrafficLights::{Red, Yellow};
use Numbers::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLights::Green;

    nested_modules();
    println!("red = {:?}, yellow = {:?}, green = {:?}", red, yellow, green);
    println!("one = {:?}, two = {:?}, three = {:?}", One, Two, Three);
}
