use raylib::{ffi::Rectangle, texture::Texture2D, RaylibHandle, RaylibThread};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub compressionlevel: i64,
    pub height: i64,
    pub infinite: bool,
    pub layers: Vec<Layer>,
    pub nextlayerid: i64,
    pub nextobjectid: i64,
    pub orientation: String,
    pub renderorder: String,
    pub tiledversion: String,
    pub tileheight: i64,
    pub tilesets: Vec<Tileset>,
    pub tilewidth: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub data: Vec<i64>,
    pub height: i64,
    pub id: i64,
    pub name: String,
    pub opacity: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub visible: bool,
    pub width: i64,
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    pub columns: i64,
    pub firstgid: i64,
    pub image: String,
    pub imageheight: i64,
    pub imagewidth: i64,
    pub margin: i64,
    pub name: String,
    pub spacing: i64,
    pub tilecount: i64,
    pub tileheight: i64,
    pub tilewidth: i64,
}

pub fn parse_json_to_map(filename: &str) -> Map {
    let json = fs::read_to_string(filename).expect("file not found!");
    let v: Map = from_str(json.as_str()).expect("cannot parse JSON file!");
    v
}

pub fn breakdown_tiles(data: Vec<Tileset>) -> Vec<Rectangle> {
    let mut x = 0;
    let mut y = 0;
    let mut tiles_arr: Vec<Rectangle> = vec![];

    for (index, tiles) in data.iter().enumerate() {
        if index == 0 && tiles.firstgid == 1 {
            tiles_arr.push(Rectangle {
                x: -(tiles.tilewidth) as f32,
                y: -(tiles.tileheight) as f32,
                height: 0.0,
                width: 0.0,
            })
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

pub fn load_tile_texture(rl: &mut RaylibHandle, th: &RaylibThread, filename: &str) -> Texture2D {
    let tiles_texture = rl
        .load_texture(&th, filename)
        .expect("cannot load the file!");
    tiles_texture
}
