use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
// usize is CPU-bases size per platform, mostly for collections and arrays

//clone - a deep copy
//copy overrides borrowing, instead of moving values, it saves values
//PartialEq allows == for the struct
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

//coordinates with striding
// let index = (y * WIDTH) + x;
// let x = index % WDITH;
// let y = index / WIDTH; // dividing integers rounds down
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn can_enter_tile_n(&self, start: Point, end: Point) -> bool {
        let point_diff: Point = end - start;
        let mut results: i32 = 0;
        let mut length: Vec<i32> = Vec::new();

        if point_diff.x + point_diff.y != 0 {
            // horizontal check
            if point_diff.y == 0 {
                for x in start.x..=end.x {
                    length.push(1);
                    let local_test = self.can_enter_tile(Point::new(x, end.y));
                    if local_test == true {
                        results += 1;
                    }
                }
            }

            // vertical check
            if point_diff.x == 0 {
                for y in start.y..=end.y {
                    length.push(1);
                    let local_test = self.can_enter_tile(Point::new(end.x, y));
                    if local_test == true {
                        results += 1;
                    }
                }
            }
        }

        let sum: i32 = length.iter().sum();
        results == sum
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
