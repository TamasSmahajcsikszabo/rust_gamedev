#![warn(clippy::pedantic)]
use crate::prelude::*;
use legion::world::SubWorld;


#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_output(
    ecs: &mut SubWorld)
{
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
        });
}
    }
}
