#![warn(clippy::pedantic)]
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
#[write_component(Thoughness)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
}

