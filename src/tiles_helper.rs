use std::{env::current_dir, path::PathBuf};

use raylib::math::Rectangle;
use tiled_json_rs::TileSet;

use crate::{TILES_HEIGHT, TILES_WIDTH};

pub fn breakdown_tiles(data: &Vec<TileSet>) -> Vec<Rectangle> {
    let mut tiles_arr: Vec<Rectangle> = vec![];

    for (_, tiles) in data.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;

        // if tiles first_gid is 1, not 0
        // thanks to Tiled, first tile is always empty
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

    let replaced = path.replace(target, "");

    let assets = path_utils("/assets")
        .into_os_string()
        .into_string()
        .unwrap();
    format!("{assets}/{replaced}")
}

pub fn path_utils(s: &str) -> PathBuf {
    let root = current_dir().unwrap();
    let mut p = root.into_os_string();
    p.push(s);
    p.into()
}
