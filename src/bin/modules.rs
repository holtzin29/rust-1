#![allow(unused)]

pub mod foo {
    pub fn print () {
        println!("foo");
    }
}

mod my {
    use super::foo;
    pub fn call_foo () {
        foo::print();
    }
   pub fn print () {
    f();
        println!("my");
    }
    fn f () {
     super::a::print();
    }
}
pub mod a {
    #[derive(Debug)]
    pub struct S {
        pub id: u32,
        pub name: String,
    }
    pub fn print () {
        println!("a");
    }
    use crate::foo;
    pub fn call_foo () {
        foo::print();
    }
}
fn main () {
my::print();
a::print();

let s = a::S {
    id: 1,
    name: "s".to_string(),
};
println!("{:?}",s);
my::call_foo();
}