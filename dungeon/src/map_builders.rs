use crate::prelude::*;
const NUM_ROOMS: usize = 20;

//mapbuilder uses its own copy of the map and passes it to the game
//room-carving algorithms
pub struct MapBuilder {
    pub map : Map,
    pub rooms : Vec<Rect>,
    pub player_start : Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles
            .iter_mut()
            .for_each(|t| *t = tile);
        // *  is a de-reference: modification happens 
        // on the referenced variable not the reference
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && 
                        p.x < SCREEN_WIDTH && 
                            p.y > 0 && 
                            p.y < SCREEN_HEIGHT {
                                let ids = map_idx(p.x, p.y);
                                self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
        }
    }
}
