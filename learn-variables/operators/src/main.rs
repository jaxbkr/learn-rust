#![allow(unused)]

fn main() {
    // +, -, *, /
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32 = a + b;
    let c = a - b;
    let c = a * b;
    let c = a / b; // division of integers round down (drop the decimal)

    println!("{a} / {b} = {c}");

    // % (remainer != mod operator)
    let a = -1;
    let b = 2;

    let rem = a % b;

    println!("{a} % {b} = {rem}");

    // literals

    let a = 1i32;
    let b = 3u64;
    let c = 1.23e3; // 1230
    let d = 1_000_000u32;

    // booleans

    let a = true && false;
    let a = true || false;
    let a = !true;

    // bitwise
    // a = 0000 0101
    let a: u8 = 5;
    // b = 0000 0011
    let b: u8 = 3;

    println!("a & b = {:08b}", a & b);
    println!("a | b = {:08b}", a | b);

    // xor
    println!("a ^ b = {:08b}", a ^ b);

    // not

    println!("!a = {:08b}", !a);

    // shifting

    println!("1 << 3 = {:032b}", 1u32 << 3);
    println!("16 >> 2 = {:032b}", 16u32 >> 2);
}
