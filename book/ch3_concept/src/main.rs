#![warn(clippy::pedantic)]
use std::io;
const MAX_LEVEL: u32 = 100_000;
// constants are created with const
// their type must be always annotated
// they can NEVER be mutable, mut cannot be called upon them
// no function can result in constants, and constants cannot be result of runtime computations
// they are available at all levels, an can be declared at any levels


const RESULT: u16 = 24;
const MAX_POINTS: i32 = 100_000;  // _ in numbers for readability

fn main() {
    authenticate();
    let mut x = 5;
    println!("The value of x is: {}", x);
    let x = 8;
    // x = 4; - won't run
    println!("The value of new x is: {}", x);
    println!("Max level is: {}", MAX_LEVEL);
    let x = x * 3; // shadowing works with let for immut. variables
    println!("Shadowing is working: {}", x == RESULT);
    println!("Max available points are: {}", MAX_POINTS);

    // changing tpye with shadowing
    // it wouldn't have run for mut variables
    let space = "   ";
    let space = space.len();
    println!("# of spaces is {}", space);

    // a variable's type cannot be mutated
    // let mut spaces = "   
    // spaces = spaces.len(); //won't work, with let it would work, we create a new variable with another type

    // boolean = 1 byte in size

    // 1 char = 4 bytes in size
    // '' is char literals are with single quote
    // "" is for string literals
    
    // Compound types - grouping multiple values into one type
    // 1. tuples
    // cannot grow/shrink once declared
    let tup: (i32, f32, bool) = (12, 12.3, true);

    // destructuring the singe tuple object bind all values
    let (x, y, z) = tup;
    println!("The boolean in the tuple is {}", z);
    println!("The middle value of tup is {}", tup.1);
    
    //2. arrays
    // a single chunk of memory allocated on stack, not on heap
    // same type with fixed length
    // vectors can grow or shrink
    let attributes: [&str; 5] = ["might", "perception", "intuition", "dexterity","determination"];
    // indexing
    // println!("Frist attribute is {}", attributes[0]);
    // println!("Enter an index!");
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("Failed to read the line!");
    // let index: usize = index.trim().parse().expect("Not a number!");
    // let element = a[index];
    // println!("The value of the element at index {} is {}", index, element);
    indexing();
    let username = get_name();
    println!("Welcome {}", username);

    // statements don't return values
    // expressions do
    // such as {} blocks, new scope definitions, macros, single values representing themselves, calling functions


}

fn get_name()->String {
    println!("What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read line!");
    //let name: String = name.trim().expect("Error");
    name
}


fn indexing() {
    let a = [1, 2, 3, 4, 5];
    println!("Enter an index!");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read the line!");
    let index: usize = index.trim().parse().expect("Not a number!");
    // Rust doesn't try to convert a non-Boolean to Boolean
    // because if is an expression it can be assigned to a varible with let
    if index <= a.len() {
        let element = a[index];
        println!("The value of the element at index {} is {}", index, element);
    } else {
        println!("Indexing out of bound.")
    }
}

fn authenticate()-> String{
    let mut msg = String::new();
    let token = String::from("Tomi");;
    loop{
        println!("\r What's the magic word?");
        let mut magic = String::new();
        io::stdin().read_line(&mut magic).expect("Failed to read the line!");
        println!("Input: {}", magic);
        let test: bool = magic.eq(&token);
        println!("Token: {}", token);
        if test {
            msg = String::from("You are authenticatd");
            break msg;
        }
    }
}