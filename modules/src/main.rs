#![allow(unused)]

use modules::my;

fn main() {
    my::print();
    my::a::print();

    let s = my::a::S {
        id: 1,
        name: "Hello".to_string(),
    };

    println!("{:?}", s);

    my::call_foo();
    my::a::call_foo();
}
