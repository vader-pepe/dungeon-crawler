use raylib::math::Rectangle;
use tiled_json_rs::TileSet;

use crate::{TILES_HEIGHT, TILES_WIDTH};

pub fn breakdown_tiles(data: &Vec<TileSet>) -> Vec<Rectangle> {
    let mut tiles_arr: Vec<Rectangle> = vec![];

    for (index, tiles) in data.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;

        let current_pointer = index;
        let previous_pointer = if index > 0 {
            current_pointer - 1
        } else {
            current_pointer
        };
        if current_pointer != previous_pointer + 1 {
            x = 0;
            y = 0;
        }
        if tiles.first_gid == 1 {
            tiles_arr.push(Rectangle {
                x: -TILES_WIDTH as f32,
                y: -TILES_HEIGHT as f32,
                height: 0.0,
                width: 0.0,
            });
        }
        for _ in 0..tiles.tile_count {
            tiles_arr.push(Rectangle {
                x: x as f32,
                y: y as f32,
                width: tiles.tile_width as f32,
                height: tiles.tile_height as f32,
            });
            x += tiles.tile_width;
            if x >= (tiles.columns * tiles.tile_width) {
                x = 0;
                y += tiles.tile_height;
            }
        }
    }

    tiles_arr
}

pub fn pathname_format_for_maps(path: &str) -> String {
    let target = "../";
    let replacement = "./src/assets/";

    path.replace(target, replacement)
}
