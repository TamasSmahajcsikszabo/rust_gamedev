#![warn(clippy::pedantic)]
use bracket_lib::prelude::{ColorPair, to_cp437};

// a module to spawn entities
pub use crate::prelude::*;

// components are pushed as a tuple
// push creates a new entity
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player, //tag
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}
