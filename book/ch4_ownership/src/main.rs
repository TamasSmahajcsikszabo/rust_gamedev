// stack and heap
// stack: fixed size elements, LIFO, pushing onto the stack, and popping off
// heap: unorganized, pointers to locations of allocated memory, allocation, requires more work

// ownership rules: 
// 1. each value has an owner variable
// 2. there can be only one owner
// 3. when the owner goes out of scope, the value is dropped

// String - as opposed to previous types, stored in heap
// string literals are immutable, known length at compulation time
// String is mutable

fn main() {
    let mut s=String::from("Hello world!"); //String::from allocates memory when compiled
    s.push_str(" Again!");
    println!("{}", s);

    let x = 5;
    let y = x; // literally maked hard copy by default, as stored in stack 

    // with Strings, just the stack (pointer, length, capacity) are copied
    //let j = s;
    // println!("{}", s); // will throw error, as s as a reference has been invalidated!
    // thus assigning j to s is called *move* ~ shallow copy with invalidation
    // deep copy can be achieved with
    let mut j = s.clone();
    j.push_str(" - a clone");
    println!("{}", j);
    // Copy trait
    
    //functions
    take_ownership(j);
    make_copy(x);
    //println!("{}", j); // throws error because j is moved and dropped by take_ownership()

    let mut g = gives_ownership("silver bells");
    let (g, ln)= calculate_length(g);
    println!("{} is {} char long", g, ln);

    // referencing and borrowing
    // just using g's reference without owning it
    let l= calculate_lengthr(&g);
    println!("{}", l);
    
    // mutable references
    let mut n = String::from("Hello");
    let mut n2 = String::from(" Mutable references!");
    let m = concat(&mut n, &mut n2);
    println!("{}", m);
    // one scope can only have one mutable reference to the same variable
    // 1. make n mutable
    // 2. borrow it with &mut n
    // 3. accept it in the function with str: &mut String

    // immutable reference blocks future mutable for the same variable
    // multiple immutable references are in order

    // Slices
    let mut s=String::from("Hello Bello");
    let w1=&s[0..5];
    println!("{}", w1);

    let mut s = String::from("TEsting");
    first_word(&s[..]); // for Strings
    // string literals are already string slices



} // Rust calls drop() to garbage collect memory as s is no longer in scope

fn take_ownership(some_string: String){
    println!("{}", some_string);
}

fn make_copy(some_integer: i32){
    println!("{}",some_integer);
}

fn gives_ownership(txt: &str) -> String {
    let mut some_string = String::from(txt);
    println!("{}", some_string);
    some_string
}

fn calculate_length(string: String) -> (String, usize) {
    let mut ln: usize = string.len();
    (string, ln) // without reference, the original variable need to be returned to be able use it later
}

fn calculate_lengthr(string: &String) -> usize {
    string.len()
}

fn concat(str1: &mut String, str2: &mut String) -> String{
    str1.push_str(str2);
    str1.to_string()
}
// first version - holds reference to a value no longer valid if the string is dropped...
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item==b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}