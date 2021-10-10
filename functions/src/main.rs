fn main() {
    println!("Main function...");
    another_function(5);
}

fn another_function(x: i32) {
    for _i in 0..x {
        println!("Hello from another function!\n");
    };
}
