#![allow(unused)]

trait Animal {
    fn speak(&self) -> String;
}
struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}
impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}
// static (known at compile time) static dispatch
fn greet(animal: &impl Animal) {
    println!("{}", animal.speak())
}
// returns animal
fn return_concretetet_type()  -> impl Animal {
    Dog
}
// dynamic dispatch
// &dyn
fn greet_dyn(animal: &dyn Animal) {
    println!("dynamic {}", animal.speak())
}
// cannot return a reference(outlib function) so use box with dyn since not known at compile time and impl the trait animal
fn rand_animal(rand: u32) ->  Box<dyn Animal> {
    if rand <= 10 {
        Box::new(Dog) // box stores in the heap
    }
    else {
        Box::new(Cat)
    }
}
// not known at compile time
fn main() {
   let cat = Cat;
   let dog = Dog;
   greet(&cat); // static known at compile time
   greet(&dog); 
   let animal =return_concretetet_type();
   // speak with dog 
   println!("animal {}", animal.speak()); 

   let animal_str = "Dog";
   // not known at compile time
   let animal: &dyn Animal= match animal_str {
    "dog" => &Dog,
    _ => &Cat,
   };
   greet_dyn(animal);

   let animal = rand_animal(11);
   println!("animal = {:?}", animal.speak());
}