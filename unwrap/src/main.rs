#![allow(unused)]

fn main() {
    let x: Option<i32> = Some(5);

    let v = x.unwrap(); // panics if None
    println!("val = {v}");


    let y: Option<i32> = Some(2);
    let v = y.expect("y is none");
    println!("val = {v}");

    let x = 1;
    let y = 1;
    let z: Result<u32, String> = Err("Div by zero".to_string());

    let v = z.unwrap();

    println!("val = {v}");

}