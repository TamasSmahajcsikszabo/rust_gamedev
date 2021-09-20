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

    // let mut enemy_pos = Point::zero();
    // let mut other_enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    // other_enemies.iter(ecs).for_each(|(_,pos)| enemy_pos = *pos);


    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos) //** removes the reference
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });

    // let mut rng = RandomNumberGenerator::new();
    // let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    // enemies
    //     .iter(ecs)
    //     .filter(|(entity, pos)| **pos == enemy_pos) //** removes the reference
    //     .for_each(|(entity, pos)| {
    //         commands.push(spawn_monster(&mut ecs, &mut rng, *pos));
    //     });

    // let mut players = <&mut i32>::query().filter(component::<Player>());
    // players.iter_mut(ecs).for_each(|score| {
    //     let score_value: i32 = match *score.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => 0,
    //     };

    //     println!("{}",score_value);
    // }
    // );
}
