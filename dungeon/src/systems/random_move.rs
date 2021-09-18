#![warn(clippy::pedantic)]

use crate::prelude::*;
use legion::world::SubWorld;
// use std::{thread, time};

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    // let mut rng = RandomNumberGenerator::new();
    // let random_wait_time = time::Duration::from_millis(rng.range(100,300));
    // thread::sleep(random_wait_time);
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers.iter_mut(ecs).for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let multiplier = rng.range(1,4);
        let destination = match rng.range(0, 75) {
            0 => Point::new(-1*multiplier, 0),
            1 => Point::new(multiplier, 0),
            2 => Point::new(0, -1 * multiplier),
            3 => Point::new(0, multiplier),
            _ => Point::new(0, 0),
        } + *pos;
        
        if map.can_enter_tile(destination) && map.can_enter_tile_n(*pos, destination) {
            *pos = destination;
        }
    });
}
