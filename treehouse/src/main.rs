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
//loops: they run code inside a block until *break* is encountered

// enumerations: predefined set of values, can be any type even functions
// can derive functionality, just like structs
// e.g. Debug allows them to be print their values by name
// like struct and implicit return declarations, doesn't end with colon
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
// the Debug trait lets any type to printed
// it will allow println! with {:?} to print the entire struct
// struct field values can be any, other sructs, too
// i8 ~ 8 bit signed integer
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action : VisitorAction, age : i8) -> Self {
        // the lack of semicolon denotes "implicit return" syntax
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    //option2 is *destructuring* of 'note'
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}!", self.name);
                println!("{}", note);
                if self.age<21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut visitor_name = String::new();
    stdin()
        .read_line(&mut visitor_name)
        .expect("Failed to read line"); //function chaining
    visitor_name.trim().to_lowercase()
}

fn main() {
    loop {
        println!("What's your name? (Leave empty and press Enter to quit)");
        let name = what_is_your_name();
        // let visitor_list = ["bert", "steve", "fred"];
        // the alternative syntax with type and size declarations, otherwise inferred by Rust
        // using the vec! macro for Vectors
        let mut visitor_list = vec![
            Visitor::new("bert", VisitorAction::AcceptWithNote{note: String::from("Hello Bert, enjoy your treehouse.")}, 45),
            Visitor::new("steve", VisitorAction::Accept, 19),
            Visitor::new("fred", VisitorAction::Refuse, 11),
        ];
        // println!("Hello {:?}", visitor_name);
        // println!("Hello {}", name);

        // searching with an iterator
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        // find() returns an Option
        // Options have either Some(x) or None values
        //
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            // fat arrow => is for executing code for matches
            None => {
                if name.is_empty() {
                    //jumping to the end of the loop
                    break;
                } else {
                    println!("{} is not on the visitor list. Please leave.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                    println!("The final list of visitors:");
                    println!("{:#?}", visitor_list);
                }
            }
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
}
