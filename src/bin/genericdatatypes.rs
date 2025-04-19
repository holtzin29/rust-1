#![allow(unused)]

enum Optionu32 {
    Some(u32), 
    None,
}

enum Optioni32 {
    Some(i32), 
    None,
}
// instead of these two,
enum Option<T> {
    Some(T),
    None,
} // FLOAT, I32, U32 any t 

enum Result<T,E> {
    Ok(T),
    Err(E),
}
// defaults to u32 if no value 
struct Point<T = u32> {
    x: T,
    y: T,
}
fn main () {
    // generic data types // option, result, vectors

    let x :Option<u32> = Option::Some(1);
    let x :Option<i32> = Option::Some(-1);

    let y: Result<bool, String> =  Result::Ok(true);

    let v = vec![1,2,3];
    let v: Vec<u32> = vec![1,2,3];
    let v: Vec<i32> = vec![1,2,3];
    let v: Vec<_> = vec![1,2,3];

    let p0 = Point {
        x:0,
        y: 0
    };
    let p1: Point<u32> = Point {
        x: 1,
        y: 2
    };

}