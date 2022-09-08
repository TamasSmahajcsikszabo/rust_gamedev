#![warn(clippy::all, clippy::pedantic)]
fn main() {
    let four = IPAddressKind::V4(String::from("127.0.0.1"));
    let six = IPAddressKind::V6(String::from("::1"));
    println!("{:?}", six);
    // route(IPAddressKind::V6);

    // let home = IPAdr{
    //     kind: IPAddressKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IPAdr{
    //     kind: IPAddressKind::V6,
    //     address: String::from("::1"),
    // };

    // println!("Home address is {:?}", home);
    // println!("Loopback address is {:?}", loopback);
    let m = Message::Write{message: String::from("Hello")};
    m.call();

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(USState::Alaska));
    plus_one(Some(5));
    plus_one(None);
}

#[derive(Debug)]
enum IPAddressKind {
    //variants of the enum:
    V4(String),
    V6(String)
}

#[derive(Debug)]
struct IPAdr {
    kind: IPAddressKind,
    address: String,
}

// fn route(id_kind: IPAddressKind) {}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x:u32, y:u32},
    Write {message: String},
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

// impl std::fmt::Display for Message {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         println!("{}", self)
//     }
// }

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime=> 10,
        Coin::Quarter(state)=> {
            println!("The coin is from {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Arkansas,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
