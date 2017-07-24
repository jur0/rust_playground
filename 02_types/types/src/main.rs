
fn main() {
    // _ can be placed anywhere in a number literal.
    let decimal = 100_321;
    // 0x - hexadecimal literal
    let hex = 0xffff;
    // 0o - octal literal
    let octal = 0o77;
    // 0b - binary literal
    let binary = 0b1111000;
    // b'A'- byte value (u8) literal, ASCII code for A character
    let byte = b'A';
    println!("decimal = {}, hex = {}, octal = {}, binary = {}, byte = {}",
             decimal, hex, octal, binary, byte);

    // If no type is specified, the default types (usually) are i32 or f64.
    let default_int = 50; // i32
    let default_float = 10.5498; // f64
    let uint64: u64 = 5000;
    let size: usize = 10000;
    println!("default_int = {}, default_float = {}, uint64 = {}, size = {}",
             default_int, default_float, uint64, size);

    // Literals' type can be determined using 'uX', 'iX' at the end of the
    // literal.
    let iint16 = 290i16;
    // Note that _ can be anywhere
    let uint8 = 10_u8;
    println!("iint16 = {}, uint8 = {}", iint16, uint8);

    let boolean = false;
    println!("boolean = {}", boolean);

    // Char is always UTF-8 (32 bits)
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("z = {}, cat = {}", z, heart_eyed_cat);

    // #[derive(Debug)] - to be able to use {:?} in println!.
    #[derive(Debug)]
    let unit = ();
    println!("unit = {:?}", unit);

    let i32_max = std::i32::MAX;
    // In debug build Rust checks for arithmetic overflow and panics!
    // In release build this would wrap to a negative number.
    //let i32_max_inc = i32_max + 1;
    // Wrapping works in debug and release mode with the following function:
    let i32_max_inc = i32_max.wrapping_add(1);
    println!("i32_max_inc = {}", i32_max_inc);

    #[derive(Debug)]
    struct S1 {
        x: i32,
        y: i32,
    };
    let s1 = S1 { x: 10, y: 20 };
    println!("s1 = {:?}", s1);

    #[derive(Debug)]
    struct S2(i32, i32);
    let s2 = S2(10, 20);
    println!("s2 = {:?}", s2);

    #[derive(Debug)]
    struct S3;
    let s3 = S3;
    println!("s3 = {:?}", s3);

    #[derive(Debug)]
    enum E {
        X,
        Y(i32),
    };
    let e1 = E::X;
    let e2 = E::Y(100);
    println!("e1 = {:?}, e2 = {:?}", e1, e2);

    #[derive(Debug)]
    let b = Box::<(i32, u32)>::new((1, 2));
    println!("b = {:?}", b);

    let a = 1_000i64;
    // Immutable reference
    let ra = &a;
    println!("ra = {}, *ra = {}", ra, *ra);

    let mut b = 2_000u64;
    let mut rb = &mut b;
    *rb = *rb + 1;
    println!("rb = {}, *rb = {}", rb, *rb);

    // rs1 (rs2) is a reference to a string allocated on heap
    let rs1 = String::from("this is string");
    let rs2 = "another string".to_string();
    println!("rs1 = {}, rs2 = {}", rs1, rs2);

    // rstr is of type &str - a reference to a text which is part of resulting
    // binary.
    let rstr = "some string";
    println!("rstr = {}", rstr);

    // Fixed-length array
    let array = [1, 2, 3, 4, 5];
    // Reference to slice
    #[derive(Debug)]
    let slice_array1= &array[1..4];
    let slice_array2= &array[..2];
    println!("slice_array1 = {:?}, slice_array2 = {:?}", slice_array1, slice_array2);

    // Dynamically sized array - vector
    #[derive(Debug)]
    let v:Vec<f64> = vec![0.1, 0.34, 1.904, 0.432];
    println!("v = {:?}", v);
}
