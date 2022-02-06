// structs
// like tuples, elements can be of different types

fn main() {
    let guard = Character{
        // we use owned tpyes rather than references as no lifetimes are defined
        name: String::from("regular guard"),
        origin: String::from("Central"),
        level: 3,
        experience: 150,
        playable: false,
    };
    println!("Character name: {}", guard.name);

    let guard2 = Character{
        name: String::from("leader"),
        // struct update syntax
        ..guard
    };
    println!("Character name: {}", guard2.name);
    let params=Stats(0,100,122);
}

struct Character {
    name: String, 
    origin: String,
    level: i16,
    playable: bool,
    experience: i32
}

fn spawn_character(name: String, level: i16, experience: i32, origin: String) -> Character {
    Character{
        name: String::from(name),
        // field init shorthand
        level,
        experience,
        origin: String::from(origin),
        playable: false,
    }
}

// tuple struct examples
// this creates a new type different of another struct tuple with the same elements
struct Stats(i32, i32, i32);


// unit-like structs
struct Event();
