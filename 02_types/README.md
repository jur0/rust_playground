# `types`

  * `i8`, `i16`, `i32`, `i64` - signed integer
  * `u8`, ... - unsigned integer
  * `isize`, `usize` - machine word size, same as an address on a machine
  * `f32`, `f64` - float
  * `bool`
  * `char` - unicode character (32 bits!)
  * `()` - unit type (empty tuple)
  * `(i32, bool, char)` - tuple
  * `struct S { a: i32, b: u16 }` - named-field struct
  * `struct S ( i32, f64 )` - tuple-like struct
  * `struct S` - unit-like struct (no fields)
  * `enum Phase { Running(i32), Stopped }` - enumeration
  * `Box<u64>` - owning pointer to value in heap
  * `&i32`, `&mut i32` - shared and mutable references
  * `String` - UTF-8 string, dynamically sized
  * `&str` - reference to `str` (pointer to UTF-8 text)
  * `[i32; 10]` - fixed length array
  * `Vec<u16>` - vector, dynamically sized array
  * `&[i32]`, `&mut [i32]` - (mutable) reference to slice
  * `&Write`, `&mut Read` - trait object
  * `fn (isize, &str) -> isize` - pointer to function
