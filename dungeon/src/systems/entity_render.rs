#![warn(clippy::pedantic)]

use legion::world::SubWorld;

use crate::prelude::*;

#[system]
#[read_component(Point)] // where the entity is
#[read_component(Render)] // describes entitiy appearance
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // camera is used to compute offset

}
