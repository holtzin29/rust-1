#![allow(unused)]

fn f<T: std::fmt::Debug>(t: T) {
    // println!("{:?}", t); this does not compile because it needs to impl debug
   println!("{:?}", t);
}
trait A {

}
trait B {
    
}
trait C {
    
}
impl A for u32 {

}
impl B for u32 {

}
impl C for u32 {

}
impl A for i32 {

}
// TYPE T ALSO HAS TO IMPLEMENT A
fn c<T: A> (x: T) {

}
// Needs to impl both a and b
fn m<T: A + B> (x: T) {

}
fn n<T, U> (x: T, y: U) 
where 
T: A + B,
T: B + C,
{}
// difference between impl trait syntax and trait bounds
// x and y can be different type
fn k(x: impl A, y: impl A) {

}
// x and y must be the same type(T)
fn g<T:A>(x: T, y: T) {

}
// can be of different types
fn h<T: A, U: A> (x: T, y: U) {

}
fn main()  {
 let u: u32 = 1;
 let i: i32 = -1;
 let f: f32 = -0.01;

 c(u);
 c(i);
 // c(f); f does not impl trait A
 m(u);
// m(i);
  
 n(u,u);
 n(u, i);

 k(u,i);
 g(u, u);
 g(i,i);
 // g(i,u);

 h(u, i);
}