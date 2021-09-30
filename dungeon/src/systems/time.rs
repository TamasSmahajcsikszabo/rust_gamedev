#![warn(clippy::pedantic)]
use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use std::ops::Rem;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomlyByTime)]
pub fn random_move_bytime(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] time: &f32,
) {
    let mut rng = RandomNumberGenerator::new();
    let threshold = rng.range(10.0, 40.0);
    let remainder = time % 1000.0;
    let test = remainder as f32 <= threshold;
    if test {
        let mut movers = <(Entity, &Point, &MovingRandomlyByTime)>::query();
        movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let intent = rng.range(0, 4);
            if intent == 0 {
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
                // _ => Point::new(0, 0),
            } + *pos;

            commands.push((
                (),
                WantToMove {
                    entity: *entity,
                    destination,
                }));
        }});
    }
}
