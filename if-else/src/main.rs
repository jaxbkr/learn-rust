#![allow(unused)]

fn main() {
    let x: i32 = 10;

    // standalone

    if x % 2 == 0 {
        println!("{x} is even");
    } else {
        println!("{x} is odd");
    }

    // if-else that returns a value

    let z: i32 = if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    };

    println!("z = {z}")
}
