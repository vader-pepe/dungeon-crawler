use raylib::ffi::Rectangle;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub compressionlevel: i32,
    pub height: i32,
    pub infinite: bool,
    pub layers: Vec<Layer>,
    pub nextlayerid: i32,
    pub nextobjectid: i32,
    pub orientation: String,
    pub renderorder: String,
    pub tiledversion: String,
    pub tileheight: i32,
    pub tilesets: Vec<Tileset>,
    pub tilewidth: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: String,
    pub width: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub data: Vec<i32>,
    pub height: i32,
    pub id: i32,
    pub name: String,
    pub opacity: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub visible: bool,
    pub width: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    pub columns: i32,
    pub firstgid: i32,
    pub image: String,
    pub imageheight: i32,
    pub imagewidth: i32,
    pub margin: i32,
    pub name: String,
    pub spacing: i32,
    pub tilecount: i32,
    pub tileheight: i32,
    pub tilewidth: i32,
}

pub fn parse_json_to_map(filename: &str) -> Map {
    let json = fs::read_to_string(filename).expect("file not found!");
    let v: Map = from_str(json.as_str()).expect("cannot parse JSON file!");
    v
}

// TODO: didn't account for different size tiles. fix later.
// also have weird bugs
pub fn breakdown_tiles(data: &Vec<Tileset>) -> Vec<Rectangle> {
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
        if tiles.firstgid == 1 {
            tiles_arr.push(Rectangle {
                x: -(tiles.tilewidth) as f32,
                y: -(tiles.tileheight) as f32,
                height: 0.0,
                width: 0.0,
            });
        }
        for _ in 0..tiles.tilecount {
            tiles_arr.push(Rectangle {
                x: x as f32,
                y: y as f32,
                width: tiles.tilewidth as f32,
                height: tiles.tileheight as f32,
            });
            x += tiles.tilewidth;
            if x % tiles.columns == 0 {
                x = 0;
                y += tiles.tileheight;
            }
        }
    }

    tiles_arr
}

pub fn format_pathname(path: &str) -> String {
    if let Some(first_slash_idx) = path.find('/') {
        if let Some(second_slash_idx) = path[first_slash_idx + 1..].find('/') {
            let start_index = first_slash_idx + second_slash_idx + 1;
            let result = &path[start_index..].to_string();
            let formatted = format!("./src{result}");
            return formatted;
        }
    }
    String::new()
}
