#![allow(unused)]
use std::{collections::HashSet, hash::Hash};

fn main() {
    // hashset
  let mut set: HashSet<u32> = HashSet::new();
  let inserted = set.insert(1); // returns boolean
  println!("inserted {inserted}");

  let inserted = set.insert(1); // returns boolean
  println!("inserted {inserted}"); // return false -> 1 already exists

  // get data out
  // as input we pass reference to type
  let contains =set.contains(&1);
  println!("contains {contains}");
  let contains =set.contains(&2);
  println!("contains {contains}");
  let contains =set.contains(&3);
  println!("contains {contains}");


}