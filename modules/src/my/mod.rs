use super::foo;

pub fn call_foo() {
    foo::print();
}

pub fn print() {
    println!("my");
    private();
}

fn private() {
    a::print();
}

pub mod a;