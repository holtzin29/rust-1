#![allow(unused)]
// size
// size trait is known at compile time
/// implemented for primitive types
/// necessary for allocating on the stack
/// 
/// ?size
/// size may not be known at compile time
/// dyanmic size types
fn f<T: Sized>(x: T) {

}
// dynamic size need to be referenced or smart pointer
fn g<T:?Sized> (x: &T) {

}
trait A {

}
impl A for u32 {
    
}
fn d(x: Box<dyn A>) {}
fn main() {
    // primitive types
    let i: i64 = -1;
    let x: f64 = 0.01;
    let b: bool = true;

    f(i);
    f(x);
    f(b);
    // size of them is known at compile time

    struct S {
        i: i32, 
        j: i32,
    };
    let s = S {
        i: 1, j: 2
    };
    let arr: [i32; 4] = [1,2,3,4];
    f(arr);
    f(&arr); // size of reference also known at compile time

    // slices are ?sized
    let slice: &[i32] = &[1,2,3];
    g(slice);

   // string slice
   let s = "hello";
   g(s);

   // trait objects are also not known at compile time sort of 
   let v: Box<dyn A> = Box::new(1u32); // trait object
   // dyn a can be any type (size not known at compile)
   g(&v);


}