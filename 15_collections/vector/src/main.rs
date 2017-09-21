
fn main() {
    {
        // Empty vector using vec! mactro.
        let numbers: Vec<u32> = vec![];
        // Vector of &str.
        let words = vec!["aaa", "bbb", "ccc"];
        // 10 zeroed-out bytes.
        let buffer = vec![0u8; 10];

        println!("numbers = {:?}, words = {:?}, buffer = {:?}",
            numbers, words, buffer);

        let first_word = words[0];
        let buffer_slice = &buffer[2..5];
        // fn to_vec(&self) -> Vec<T> where T: Clone
        let buffer_slice_copy = buffer[1..3].to_vec();

        println!("first_word = {}, buffer_slice = {:?}, buffer_slice_copy = {:?}",
            first_word, buffer_slice, buffer_slice_copy);

        if let Some(word) = words.first() {
            println!("firts word = {}", word);
        }
    }

    {
        let v1: Vec<i32> = vec![];
        let mut v2 = vec![1, 2, 3];

        assert!(v1.is_empty());
        assert!(v1.len() == 0);
        assert!(!v2.is_empty());

        println!("v1.capacity() = {}", v1.capacity());
        println!("v2.capacity() = {}", v2.capacity());

        v2.push(4);
        v2.push(5);
        v2.pop();

        println!("v2.capacity() = {}", v2.capacity());
    }
}
