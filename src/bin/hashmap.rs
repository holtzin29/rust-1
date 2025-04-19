#![allow(unused)]
use std::collections::HashMap;
fn main() {
    // initialize
    let mut score: HashMap<String, u32> = HashMap::new(); // string -> name, u32 -> score
    score.insert("red".to_string(), 100); // inserting data into hashmap
    score.insert("green".to_string(), 100);

    // get
    let val: Option<&u32>= score.get("green"); // get score of green team (return value is a option)
    println!("val = {:?}", val);

    let val: Option<&u32>= score.get("yellow"); // none -> doesn't exist on hash map
    println!("val = {:?}", val);

    // overwrite
    score.insert("green".to_string(), 200);
    let val: Option<&u32> = score.get("green");
    println!("val = {:?}", val);
// upsert
   let v: &mut u32 =  score.entry("blue".to_string()).or_insert(0); // if it has this value, will insert 0
   *v += 200;

   let val: Option<&u32> = score.get("blue");
   println!("val {:?}", val); 

   let v: &mut u32 =  score.entry("blue".to_string()).or_insert(0);
   *v += 200;

   let val: Option<&u32> = score.get("blue");
   println!("val {:?}", val); 

}