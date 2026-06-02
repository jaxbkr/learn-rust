#![allow(unused)]

fn main() {
    // generic unconditional loop
    let mut i = 0;
    loop {
        if i > 5 {
            break;
        }
        println!("loop = {i}");
        i += 1;
    }

    // while loop
    let mut i = 0;
    while i <= 3 {
        println!("while {i}");
        i += 1;
    }

    // for loop
    for i in 0..=5 {
        println!("for loop {i}");
    }

    // for loop array

    let arr = [1, 2, 3];
    for a in arr {
        println!("array {a}");
    }

    // usize and range
    let n = arr.len();
    for i in 0..n {
        println!("array {}", arr[i]);
    }

    // for loop vector
    let v = vec![1, 2, 3];


    // iter
    for x in v.iter() { // anything implemented with iter can be looped in this way
        println!("vector {x}");
    }

    for x in v {
        println!("vector {x}");
    }

    // returning from a loop can only be done from the loop { } syntax

    let mut i = 0;
    let z = loop {
        if i == 3 {
            break 99;
        }
        i += 1;
    };
    println!("loop z returns = {z}");

    // labels

    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");
            if i == 1 && j == 2 {
                break 'outer;
            }
        }
    }

    
}
