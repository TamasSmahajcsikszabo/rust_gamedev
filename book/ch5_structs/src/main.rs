// structs
// like tuples, elements can be of different types

fn main() {
    let guard = Character{
        name: String::from("regular guard"),
        origin: String::from("Central"),
        level: 3,
        experience: 150,
        playable: false,
    };
    println!("Character name: {}", guard.name);
}

struct Character {
    name: String, 
    origin: String,
    level: i16,
    playable: bool,
    experience: i32
}

fn spawn_character(name: String, level: i16, experience: i32, origin: String) {
    Character{
        name=String::from(name),
        level,
        experience,
        origin=String::from(origin),
        playable: false,
    }

}