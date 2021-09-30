#![warn(clippy::pedantic)]

pub use crate::prelude::*;
pub use legion::systems::CommandBuffer;
pub use legion::world::SubWorld;

#[system(for_each)]
#[read_component(Player)]
#[write_component(Time)]
pub fn movement(
    entity: &Entity,
    want_move: &WantToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .unwrap() // access the Option's content
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity); // remove message once delivered
}
