#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let mylist = ["One", "Two", "Three"];
    for i in &mylist {
        println!("{}", i);
    }
}
