use std::io::stdin;
// mut is mutable
//ampersand is a reference to the variable, passes access to it, not to ots copy
//this is also called 'borrowing'
//here we lend the variable with mutability permission to the calling function
//value return does not end with ;, a shorthand for return
// we have to trim \r\n from the read-in data, get rid of non-printing characters
// logical operators || && !
// the simplest list is an array, elements have to be of the same type, size cannot be changed
// Vectors provide options to grow; Vectors(vec) are dynamically resizable, they accept push()
// without recompiling
// str and Sting are two types of strings
// range not includes last element unless added like 0..=10
//:? is the debug placeholder, a.k.a. raw printing
//:#? is for pretty printing, it prints any type that support Debug trait

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        // the lack of semicolon denotes "implicit return" syntax
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut visitor_name = String::new(); 
    stdin()
        .read_line(&mut visitor_name) 
        .expect("Failed to read line"); //function chaining
    visitor_name
        .trim()
        .to_lowercase()
}

fn main() {
    println!("What's your name?");
    let name = what_is_your_name();
    // let visitor_list = ["bert", "steve", "fred"];
    // the alternative syntax with type and size declarations, otherwise inferred by Rust
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve, Your mil is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];
    // println!("Hello {:?}", visitor_name);
    // println!("Hello {}", name);

    // searching with an iterator
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    // find() returns an Option
    // Options have either Some(x) or None values
    //
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        // fat arrow => is for executing code for matches
        None => println!("You are not on the visitor list. Please leave.")
    }

    // let mut allow_them_in = false;
    // for visitor in &visitor_list {
    // // it could have been indexed too, but direct iteration is safer and cleaner
    //     if &visitor_name == visitor.name {
    //         allow_them_in = true;
    //     }
    // }

    // if allow_them_in {
    //     println!("Welcome to the Treehouse, {}!", visitor_name);
    // } else {
    //     println!("Sorry, you aren't on the list!");
    // }
}
