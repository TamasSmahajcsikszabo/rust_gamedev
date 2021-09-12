#![warn(clippy::pedantic)]
use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN: u32 = 1;
const MAX: u32 = 101;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(MIN, MAX);
    let mut point: u32 = MAX;
    let dif: u32 = 2;

loop {
    println!("Please input your guess!");

    let mut guess = String::new(); //statis method of String

    io::stdin()
        .read_line(&mut guess) // a handle to the terminal's standard input (as a stdin::Result type),
        // plus we need a mutable reference
        // stin::Result is an enum, its values are its so-called variants
        .expect("Failed to read the line!");

    // shadowing the original string:
    let guess: u32 =match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
                point = point - dif;
            }
            Ordering::Greater => {
                println!("Too big! Try guess between 1 and 100");
                point = point - dif;
            }
            Ordering::Equal => {
                println!("You win!");
                println!("Your score is {}", point);
                break;
            }
        }
    }
}
