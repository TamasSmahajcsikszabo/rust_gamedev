use std::io;

fn main() {
    let mut index = String::new();

    println!("Enter index for Fibonacci sequence!");
    io::stdin().read_line(&mut index).expect("Not a number!");

    let index: usize =match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 10
    };
    let fib = fibonacci(index);
    println!("{}", fib);
}

fn fibonacci(index: usize) -> usize{
    let mut a: usize = 0;
    let mut b: usize = 1;
    let mut fib = a + b;
    let mut i: usize = 0;

    while index-2 > i {
        i+=1;
        a = b;
        b = fib;
        fib = a + b;
        println!("i: {} fib: {}", i, fib);
    }

    fib
}