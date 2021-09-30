#![warn(clippy::pedantic)]

use legion::world::SubWorld;

use crate::prelude::*;

#[system]
#[read_component(Point)] // where the entity is
#[read_component(Render)] // describes entitiy appearance
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // camera is used to compute offset
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // < ... > indicates *tpyes* involved
    // query over a tuple, returning elements having both components
    <(&Point, &Render)>::query()
        .iter(ecs) // specify which subWorld to use
        .for_each(|(pos, render)| {
            // destructuring in action
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
