#![feature(plugin)]

#[plugin]
extern crate clippy;

pub fn test(foo: Box<Vec<bool>>) {
    println!("{:?}", foo.get(0))
}

fn main(){
    test(Box::new(Vec::new()));
}