mod player;
mod tiles_helper;
use player::{render_player, Frames, Player, PlayerTextures};
use tiles_helper::{breakdown_tiles, path_utils, pathname_format_for_maps};

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use tiled_json_rs::{Map, ObjectType};

#[derive(Debug)]
pub struct Dimension {
    SCREEN_WIDTH: i32,
    SCREEN_HEIGHT: i32,
}

const TILES_WIDTH: i32 = 16;
const TILES_HEIGHT: i32 = 16;
const SCREEN_WIDTH: i32 = 40 * TILES_WIDTH;
const SCREEN_HEIGHT: i32 = 30 * TILES_HEIGHT;
// tile size fixed 16x16 pixel
// total tiles horizontally is 40 because
// 16x40 = 640
const TILE_WIDTH_COUNT: i32 = SCREEN_WIDTH / TILES_WIDTH;
// total tiles vertically is 30 because
// 16x30 = 480
const TILE_HEIGHT_COUNT: i32 = SCREEN_HEIGHT / TILES_HEIGHT;

// TODO: setup CICD, cross compile, wasm
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let map = path_utils("/assets/maps/map_1.json");
    let map_1 = tiled_json_rs::Map::load_from_file(&map.as_path()).expect("Failed to load map");

    let mut tiles_textures: Vec<Texture2D> = vec![];
    for tileset in &map_1.tile_sets {
        let rl = &mut rl;
        let json_path = pathname_format_for_maps(
            tileset
                .image
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
                .as_str(),
        );
        let t = rl
            .load_texture(&thread, json_path.as_str())
            .expect("cannot load texture!");
        tiles_textures.push(t);
    }

    let tile_arr = breakdown_tiles(&map_1.tile_sets);

    // TODO: handle texture better
    // tips: create prefabs(game object?) like unity
    let player_torso_idle_texture = &rl
        .load_texture(&thread, "./assets/img/Heroes/Rogue/Idle/Idle-Sheet.png")
        .expect("unable to load texture!");

    let player_torso_walk_texture = &rl
        .load_texture(&thread, "./assets/img/Heroes/Rogue/Run/Run-Sheet.png")
        .expect("unable to load texture!");

    let player_hands_texture = &rl
        .load_texture(&thread, "./assets/img/Weapons/Hands/Hands.png")
        .expect("unable to load texture!");

    let weapon_texture = &rl
        .load_texture(&thread, "./assets/img/Weapons/Wood/Wood.png")
        .expect("unable to load texture!");

    let slash_texture = &rl
        .load_texture(&thread, "./assets/img/Anim/_Attack.png")
        .expect("unable to load texture!");

    let mut player_position = Vector2 {
        x: (19 * TILES_WIDTH) as f32,
        y: (27 * TILES_HEIGHT) as f32,
    };

    rl.set_target_fps(60);

    let mut body = PlayerTextures {
        torso: vec![player_torso_idle_texture, player_torso_walk_texture],
        hands: player_hands_texture,
        slash: slash_texture,
        weapon: weapon_texture,
    };

    let mut frames = Frames {
        attack_frame: &mut 0.0,
        walk_frame: &mut 0.0,
        idle_frame: &mut 0.0,
    };

    let mut player = Player::new(&mut body, &mut frames, &mut player_position);
    while !&rl.window_should_close() {
        player.process(&rl);

        //
        //     // Check player position to avoid moving outside tilemap limits
        //     if player_position.x < 0.0 {
        //         player_position.x = 0.0;
        //     }
        //
        //     if player_position.y < 0.0 {
        //         player_position.y = 0.0;
        //     }
        //
        //     if player_position.x >= (SCREEN_WIDTH as f32) - (player_torso_texture.width / 4) as f32
        //     {
        //         player_position.x = (SCREEN_WIDTH as f32) - (player_torso_texture.width / 4) as f32;
        //     }
        //
        //     if player_position.y >= (SCREEN_HEIGHT as f32) - (player_torso_texture.height) as f32 {
        //         player_position.y = (SCREEN_HEIGHT as f32) - (player_torso_texture.height) as f32;
        //     }
        // }

        {
            let mut d = rl.begin_drawing(&thread);
            draw_scene(&mut d, &map_1, &tiles_textures, &tile_arr);

            render_player(&mut d, &mut player);
        }
    }
}

// Drawing Tilemap
fn draw_scene(
    d: &mut RaylibDrawHandle,
    map: &Map,
    tiles_textures: &Vec<Texture2D>,
    tiles: &Vec<Rectangle>,
) {
    d.clear_background(Color::new(20, 20, 18, 1));
    // TODO: handle rotation
    {
        // TODO: multi threading?? lessgoo
        for (_layer_index, map_layer) in map.layers.iter().enumerate() {
            let mut x = 0;
            let mut y = 0;

            match &map_layer.layer_type {
                tiled_json_rs::LayerType::ImageLayer(_image) => {
                    todo!()
                }
                tiled_json_rs::LayerType::Group { layers: _ } => {
                    todo!()
                }
                tiled_json_rs::LayerType::TileLayer(tile_layer) => {
                    for tile_number_on_screen in 0..(TILE_WIDTH_COUNT * TILE_HEIGHT_COUNT) {
                        let tile_number_on_screen = tile_number_on_screen as usize;
                        let mut texture = &tiles_textures[0];
                        for (j, tiles) in map.tile_sets.iter().enumerate() {
                            if tile_layer.data[tile_number_on_screen] >= tiles.first_gid
                                && tile_layer.data[tile_number_on_screen]
                                    <= (tiles.first_gid + tiles.tile_count - 1)
                            {
                                texture = &tiles_textures[j]
                            }
                        }
                        d.draw_texture_rec(
                            texture,
                            Rectangle {
                                x: tiles[tile_layer.data[tile_number_on_screen] as usize].x,
                                y: tiles[tile_layer.data[tile_number_on_screen] as usize].y,
                                width: TILES_WIDTH as f32,
                                height: TILES_HEIGHT as f32,
                            },
                            Vector2 {
                                x: x as f32,
                                y: y as f32,
                            },
                            Color::WHITE,
                        );
                        x += TILES_WIDTH;
                        if x % SCREEN_WIDTH == 0 {
                            x = 0;
                            y += TILES_HEIGHT;
                        }
                    }
                }
                tiled_json_rs::LayerType::ObjectGroup(obj_group) => {
                    // TODO: map collision starts here.
                    // pain.
                    for (_obj_index, object) in obj_group.objects.iter().enumerate() {
                        match &object.object_type {
                            ObjectType::None => {
                                // TODO: fix this
                                //
                                // d.draw_rectangle(
                                //     object.x as i32,
                                //     object.y as i32,
                                //     object.width as i32,
                                //     object.height as i32,
                                //     Color::RED,
                                // );
                            }
                            _ => (),
                        };
                    }
                }
            }
        }
    }
}
