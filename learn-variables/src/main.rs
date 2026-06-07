#![allow(unused)]
use std::u128;

fn main() {
    /*** MUTABILITY AND TYPE INFERENCING ***/

    let x: i32 = -123;
    let mut y: i32 = 123;
    y += 1;
    // x += 1; This does not work because immutable by default

    const NUM: u32 = 1;

    let x: i32 = -1;
    let x: bool = true;

    let v: Vec<_> = vec![1, 2, 3];

    /*** TYPES ***/

    // Integers - i(n), range = -(2**(n-1) ~ 2**(n-2) -1)
    let i0: i8; // -128 ~ 127
    let i1: i16; // -256 ~ 255
    let i2: i32;
    let i3: i64;
    let i4: i128;
    let i5: isize; // takes the size of architecture i.e. 32-bit or 64-bit

    // Unsigned integers
    // 0 ~ 2**n - 1
    let u0: u8 = 1; // 0 ~ 255
    let u1: u16 = 1; // 0 ~ 511
    let u2: u32 = 1;
    let u3: u64 = 1;
    let u4: u128 = 1;
    let u5: usize = 1; // This type is used for indexes for arrays and vectors

    // Floats
    let f0: f32 = 0.01;
    let f1: f64 = 0.01;

    // Boolean

    let b: bool = false;

    // Chars

    let c: char = 'c';

    // Conversion of type

    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and max

    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("char min: {}", min_char as u8);
    println!("char max: {}", max_char as u8);

    // Overflow -- Note, will not panic when in release mode if overflow occurs
    let mut u: u32 = u32::MAX;
    // u += 1;
    println!("overflow u32 {u}");

    // how to check for overflows
    // checked_add - Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", u);

    // wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping: {:?}", u);

    /*** COMPOUND DATA TYPES ***/

    // tuple

    let t: (bool, u32, char) = (true, 5, 'a');

    // destructuring

    let (a, b, c) = t;

    // ignoring with _
    let (_, b, _) = t;

    // empty tuple -- unit type

    let t = (); // kinda like void

    // nested tuple

    let nested = ((3, 4), (true, true, 1u32), ());

    // how to access

    let t: (bool, u32, char) = (true, 1, 'c');
    println!("t = {}, {}, {}", t.0, t.1, t.2);

    println!("nested {} {}", nested.0.0, nested.1.2);

    // Array
    // arrays are fixed len known at compile time
    // slices are unknown len at compile time

    let mut arr: [u32; 3] = [1, 2, 3];
    println!("arr = [{},{},{}]", arr[0], arr[1], arr[2]);
    println!("{:?}", arr);

    let zero_arr: [u32; 10] = [0; 10];
    println!("{:?}", zero_arr);

    // slices

    let nums: [i32; 10] = [-1, 1, 0, 4, 3, 2, 1, 2, 4, 9];

    // first 3 elements

    let s = &nums[..3]; // 0, 1, 2
    println!("nums = {:?}", nums);
    println!("first 3 elements of nums = s = {:?}", s);

    // middle 4 elements
    let s = &nums[3..7];
    println!("middle 4 elements of nums = s = {:?}", s);

    // last 3

    let s = &nums[7..];
    println!("last 3 elements of nums = s = {:?}", s);

    // all elements

    let s = &nums[..];
    println!("all elements of nums = s = {:?}", s);

    /*** STRINGS ***/

    // String = vector of u8 (Vec<u8>)
    // &str = slice of u8 (&[u8])

    // Use String if mutation or needed ownership, use &str for read-only ops

    let msg: String = String::from("Hello Rust!");
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");

    let s: &str = &msg[0..5];
    let s_len: usize = s.len();
    println!("s: {s}");
    println!("s_len: {s_len}");

    let hello: &str = "Hello Rust"; // String literal, this is stored directly in the binary and immutable

    let s: &str = r#"
    Multi-line
    String literal
    "#;

    println!("{s}");

    // Deref coercion

    let msg: String = String::from("Hi There");
    let s: &str = &msg;

    let mut msg: String = "Hello Rust".to_string();
    msg += "!";

    println!("{msg}");

    let msg1 = "Rust";
    let msg2 = "Is Awesome";

    let msg3 = format!("{msg1} {msg2}");

    println!("{msg3}");

    /*** ENUMS ***/
    #[derive(Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
        Rgba(u8, u8, u8, f32),
        Hex(String),
        Hsl { h: u8, s: u8, l: u8 }
    };

    let color1: Color = Color::Red;
    let color2: Color = Color::Rgba(20,30,10,0.35);
    let color3: Color = Color::Hex("#ffffff".to_string());
    let color4: Color = Color::Hsl { h: 0, s: 1, l: 200 };

    println!("{:?}, {:?}, {:?}, {:?}", color1, color2, color3, color4); 

    // PartialEq

    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // Option enum ==> Some(11) | None

    let x: Option<i32> = None;
    let y: Option<i32> = Some(-23);


    // Result enum

}
