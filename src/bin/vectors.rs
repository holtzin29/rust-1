#![allow(unused)]

fn main() {
    // vector are arrays that can grow and shrink at compile time
    // vec<>
     let i: Vec<i32> = vec![-1, 0, -2];
     let v: Vec<i32> = Vec::new();
     let v: Vec<u32> = vec![1,2,3];

     let v = vec![1u8, 2,3]; // first element == u8
     let v = vec![1,2,3,4,5];
     let v = vec![1u8; 5]; // all five elements == 1, five elements
     println!("v = {:?}, length = {}", v, v.len());

     // get
     let v = vec![1,2,3];
     let x =  v[0]; // 1
     let y = v[1]; // 2
     let x = v.get(0);
     match x {
        Some(val) => println!("val = {val}"),
        None => println!("invalid index"),
     }

     // update
     let mut v = vec![1,2,3,4,5];
     v[1] = 99; 

     // add
     let mut v: Vec<i32> = Vec::new();
     v.push(1);
     v.push(2);
     v.push(3); // stack
     println!("push = {:?}", v);

     // pop -> pop the end of array
     let mut v = vec![1,2,3];
     match v.pop() {
        Some(val) => println!("val = {val}"),
        None => println!("invalid")
     }
     match v.pop() {
        Some(val) => println!("val = {val}"),
        None => println!("invalid")
      
     }
     match v.pop() {
        Some(val) => println!("val = {val}"),
        None => println!("invalid")
      
     }
     // create slice of vector
     let v = vec![1,2,3];
     let s =&v[1..4];
     println!("{:?}", s);

}