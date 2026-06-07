#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    x + y // implicit return
}

fn print() { // returns unit type
    println!("no output");
}

fn forever() -> ! { // never returns
    loop {}
}

fn crash() -> ! {
    panic!("crash and burn");
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x,y);
    println!("{x} + {y} = {z}");

    print();

    crash();
}
