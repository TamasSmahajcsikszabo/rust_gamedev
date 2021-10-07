#![warn(clippy::pedantic)]
use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    // #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    // #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        // old version with writing to the map
        // if delta.x != 0 || delta.y != 0 {
        //     let mut players = <&mut Point>::query().filter(component::<Player>());
        //     players.iter_mut(ecs).for_each(|pos| {
        //         let destination = *pos + delta;
        //         if map.can_enter_tile(destination) {
        //             *pos = destination;
        //             camera.on_player_move(destination);
        //             *turn_state = TurnState::PlayerTurn;
        //         }
        //     });
        // }
        //
        // newer version with omitting a message
        // players.iter(ecs).for_each(|(entity, pos)| {
        //     let destination = *pos + delta;
        //     commands.push((
        //         (),
        //         WantToMove {
        //             entity: *entity,
        //             destination,
        //             position: *pos
        //         },
        //     ));
        // });
        // new version with attach system
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let (player_entity, player_position) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            if !hit_something {
                commands.push((
                    (),
                    WantToMove {
                        entity: player_entity,
                        destination: destination,
                        position: player_position,
                    },
                ));
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
