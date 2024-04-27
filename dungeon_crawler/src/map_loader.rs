use either::Either::{self, Left, Right};
use raylib::RaylibHandle;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    pub columns: i32,
    pub image: String,
    pub imageheight: i32,
    pub imagewidth: i32,
    pub margin: i32,
    pub name: String,
    pub spacing: i32,
    pub tilecount: i32,
    pub tiledversion: String,
    pub tileheight: i32,
    pub tilewidth: i32,
    pub r#type: String,
    pub version: String,
}

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
    pub firstgid: i64,
    pub source: String,
}

pub enum JSONType {
    Tiles,
    Map,
}

pub fn parse_json(json_type: JSONType, filename: &str) -> Either<Tile, Map> {
    let json_data = fs::read_to_string(filename).expect("cannot open JSON file!");
    let is_tile = matches!(json_type, JSONType::Tiles);
    if is_tile {
        let v: Tile = from_str(json_data.as_str()).expect("cannot parse JSON!");
        Left(v)
    } else {
        let v: Map = from_str(json_data.as_str()).expect("cannot parse JSON!");
        Right(v)
    }
}

pub fn load_map(rl: &RaylibHandle) {}
