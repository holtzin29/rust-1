#![allow(unused)]

fn main() {
    // iter -> borrows and returns a iterator that returns a reference <T>
    // into_iter -> takes ownership and returns a iterator that may return T, &T, or &mut T // ownership is transfered inside for loop
    // iter_mut -> returns &mut T
    let mut vals: Vec<i32> = vec![1,2,3];
    for v in vals.iter() {
        println!("{}", v);
    }
    for v in vals.iter_mut() {
        *v += 1;
        println!("{}", v);
    }
    for mut v in vals.into_iter() {
        v+= 1;
        println!("{}", v);
    }
    //    println!("{}", v); can't do that
}
