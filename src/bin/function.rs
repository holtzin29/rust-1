#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    x + y
}
fn print() {
    println!("no output");
}
// never return
fn forever() -> ! {
    loop {}
}
fn crash() -> ! {
    panic!("crash");
}
fn main() {
    // function
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    // no output
    print();

    // crash
    crash();
}
