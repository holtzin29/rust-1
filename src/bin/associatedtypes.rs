#![allow(unused)]
// associated types -> placeholder type inside a trait definition
// placeholder is replaced by the implementation.

// difference between generic trait
// generic -> multiple implmentation for a trait
// associated type -> single implementation for a trait

// associated type
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
trait GenericIterator<T> {
    fn get_next(&mut self) -> Option<T>;
}

impl GenericIterator<u32> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<u32> {
        match self.array.get(self.i) {
            Some(&x) => {
                self.i += 1;
                Some(x)
            }
            _ => None,
        }
    }
}
impl Iterator for ArrayIter<u32> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.array.get(self.i) {
            Some(&x) => {
                self.i += 1;
                Some(x)
            }
            _ => None,
        }
    }
}
impl GenericIterator<bool> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<bool> {
     Some(true)
    }
}
struct ArrayIter<T> {
array: [T; 5],
i: usize,
}
fn main() {
    let mut iter = ArrayIter {
        array: [1,2,3,4,5],
        i: 0,
    };
    while let Some(x) = iter.next() {
        println!("{:?}", x);
    }
  
}
