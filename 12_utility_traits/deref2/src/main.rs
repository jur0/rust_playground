use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn compress(audio: &[u8]) -> Vec<u8> {
    println!("compress: {:?}", audio);
    // Just return something...
    vec![]
}

fn main() {
    let x = Mp3 {
        audio: vec![1, 2, 3],
        artist: None,
        title: None,
    };

    assert_eq!(vec![1, 2, 3], *x);

    // Without deref coercion, we would need to write this to call compress
    // function (we would need to write we want audio field for x struct and
    // we want slice referring to the whole Vec<u8>):
    //compress(x.audio.as_slice());

    // With deref coercion we can just write this:
    compress(&x);
    // Rust knows that Mp3 implements Deref and return &Vec<u8> from deref
    // method. Standard library implements Deref on Vec<T> to return &[T].
    // At compile time is sees it can use deref twice to turn &Mp3 to &Vec<u8>
    // and then into &[u8].
}
