#![warn(clippy::pedantic)]
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(rcs: &mut subWorld, commands: &mut CommandBuffer) {
    //commandBuffer is a container for Legion about commands after the system has performed

}
