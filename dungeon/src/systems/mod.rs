#![warn(clippy::pedantic)]

mod player_input;
mod map_render;
mod entity_render;
use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::render_map_system())
        .build()
}

