#![warn(clippy::pedantic)]

pub use crate::prelude::*;

// Clone allows copying a component
// Debug is to print debugging information
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// components are like tags
// this is to serve to indicate the player component to be the Player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

// another empty class tag for enemies
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

// a new tag component for moving randomly
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomlyByTime;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantToMove {
    pub entity: Entity,
    pub destination: Point,
    pub position: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Experience {
    pub current: i32,
    pub threshold: i32,
    pub level: i32
}


#[derive(Clone, PartialEq)]
pub struct Name(pub String); // a struct can also be a tuple

