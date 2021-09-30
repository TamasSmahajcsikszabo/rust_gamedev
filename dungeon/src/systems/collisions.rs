#![warn(clippy::pedantic)]

use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;

#[system]
#[read_component(Point)]
#[write_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    //commandBuffer is a container for Legion about commands after the system has performed
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos) //** removes the reference
        .for_each(|(entity, _)| {
            commands.remove(*entity);
            // if ecs.entry_ref(get_component::<Player>().is_ok())
            //     .unwrap()
            //     .get_component::<Player>().is_ok()
            // {
            //     Player.score += 1;
            // }
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

            // players.iter(ecs).for_each(|pos, score|)
        });
}
