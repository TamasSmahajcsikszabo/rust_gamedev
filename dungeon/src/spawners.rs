#![warn(clippy::pedantic)]
use bracket_lib::prelude::{to_cp437, ColorPair};

// a module to spawn entities
pub use crate::prelude::*;

// components are pushed as a tuple
// push creates a new entity
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // the player is crated and pushed on the map:
    ecs.push((
        Player, //tag
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health { current: 20, max: 20 },
        Experience { current: 0, threshold: 1000, level: 0 },
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    // hit point, name, glyph
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {

    let (hp, name, glyph) = match rng.roll_dice(1,10) { //destructure the tuple
        1..=8 => goblin(),
        _ => orc()
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph, 
        },
        MovingRandomly {},
        MovingRandomlyByTime {},
        Health { current: hp, max: hp },
        Name(name) //  the name component is a tuple
    ));
}
