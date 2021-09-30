#![warn(clippy::pedantic)]

use crate::prelude::*;
use legion::world::SubWorld;

#[system]
#[read_component(Health)]
#[read_component(Experience)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();
    let mut xp_query = <&Experience>::query().filter(component::<Player>());
    let player_xp = xp_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2); //targets the third layer!
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_HEIGHT*2,
        player_health.current,
        player_health.max,
        ColorPair::new(GREEN, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE,GREEN),
    );
    draw_batch.bar_horizontal(
        Point::new(0,1),
        SCREEN_HEIGHT*2,
        player_xp.current,
        player_xp.threshold,
        ColorPair::new(BLUE,BLACK),
    );
    draw_batch.print_color_centered(
        1,
        format!(" XP: {} / {}", player_xp.current, player_xp.threshold),
        ColorPair::new(WHITE,BLUE),
    );
    draw_batch.submit(10000).expect("Batch error!");
}
