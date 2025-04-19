
#![allow(unused)]

// static and dynamic dispatch
// static dispatch -> func known at compile time(monomorphazation)
// no run time cost -> no vtable lookup
/// dynamic dispatch
/// func not known at compile(only at run time)
/// vtable lookup(inside binary(code is less))
/// run time overhead
///
#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait F {
    fn f(&self);
}
impl F for A {
    fn f(&self) {
        println!("{:?}", self)
    }
}
impl F for B {
    fn f(&self) {
        println!("{:?}", self)
    }
}
// type is known at compile time
fn static_dispatch<T: F>(t: &T) {
    t.f();
}
// Fixed: Removed the unnecessary generic type parameter
fn dyn_dispatch(t: &dyn F) {
    t.f();
}
fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f();
}
fn main () {
let obj = A;
static_dispatch(&obj);  
let obj = B;
static_dispatch(&obj);  

let input = "A";
// trait object -> value that implements a trait, but its concrete type is unknown at compile time
let obj: &dyn F = match input {
   // "A" => Box::new(A),
  //   "B" => Box::new(B), // Stored in a heap by using box(pointer)
  "A" => &A,
  "B" => &B,
    _ => panic!(),
};
dyn_dispatch(obj);

let obj: Box<dyn F> = match input {
    // "A" => Box::new(A),
   //   "B" => Box::new(B), // Stored in a heap by using box(pointer)
   "A" => Box::new(A),  
   "B" => Box::new(B),
     _ => panic!(),
 };
 dyn_dispatch_box(obj); // ownership is transfered from the main func to the dyn dispatch box function
}

