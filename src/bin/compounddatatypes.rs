#![allow(unused)]

fn main() {
    // tuple
    let t: (bool, u32, char) = (true, 1, 'c');
    // destructure
    let (a, b, c) = t;
    // ignore with _
    let (_, b, _) = t;
    // empty
    let t = (); // return no data 
    // nested tuple
    let nested: ((f64, char), (bool, u32, char), ()) = ((3.14, 'x'), (true, 42, 'y'), ());
    let t: (bool, u32, char) = (true, 1, c);

    println!("nested {}", nested.0.0); // first tuple first element
}
