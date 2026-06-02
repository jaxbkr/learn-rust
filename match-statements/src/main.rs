#![allow(unsed)]

enum Animal {
    Cat,
    Dog,
    Mouse,
}


fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    match x {
        1..=10 => println!("between 1 and 10"),
        _ => println!("other"),
    }

    match x {
        i @ 1..=10 => println!("matched {i}"),
        _ => println!("other"),
    }

    let animal = Animal::Cat;
    let sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Mouse => "squeee",
    };

    println!("{:?}", sound);

    // Options and Results

    let x: Option<i32> = Some(1);
    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none"),
    }

    let res: Result<u32, String> = Err(String::from("Error detected"));
    match res {
        Ok(val) => println!("Ok {val}"),
        Err(msg) => println!("Err {msg}"),
    }

    // If let

    let x: Option<u32> = Some(232);
    if let Some(v) = x {
        println!("if let {v}");
    }
    let Some(v) = x else {
        // code must diverge (panic or return)
        panic!("x is none");
    };

    println!("v = {v}");


    
}
