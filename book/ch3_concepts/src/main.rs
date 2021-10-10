#![warn(clippy::pedantic)]
const MAX_LEVEL: u32 = 100_000;
// constants are created with const
// their type must be always annotated
// they can NEVER be mutable, mut cannot be called upon them
// no function can result in constants, and constants cannot be result of runtime computations
// they are available at all levels, an can be declared at any levels


const RESULT: u16 = 24;
const MAX_POINTS: i32 = 100_000;  // _ in numbers for readability

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 8;
    println!("The value of new x is: {}", x);
    println!("Max level is: {}", MAX_LEVEL);
    let x = x * 3;
    println!("Shadowing is working: {}", x == RESULT);
    println!("Max available points are: {}", MAX_POINTS);

    // a variable's type cannot be mutated
    // let mut spaces = "   ";
    // spaces = spaces.len(); //won't work, with let it would work, we create a new variable with another type

    // boolean = 1 byte in size

    // 1 char = 4 bytes in size
    // '' is char literals are with single quote
    // "" is for string literals
    

    // tuples
    // cannot grow/shrink once declared

}
