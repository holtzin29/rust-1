#![allow(unused)]

fn main () {
  let x: Option<i32> = None;
  let v = x.expect("x is none"); // custom error
  println!("val = {v}");
  let x: Option<i32> = Some(3);
  let v: i32 = x.unwrap();
  println!("val = {:?}", v);

  // result
  let x = 1;
  let y = 2;
  let z:    Result<u32, String> = Ok(x / y);
  let v =z.unwrap();
  println!("v {}", v);

  let x = 1;
  let y = 2;
  let z:    Result<u32, String> = Err("Div by 0".to_string());
  let v =z.unwrap();
  println!("v {}", v);
}