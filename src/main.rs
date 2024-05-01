mod tiles_helper;
use tiles_helper::{breakdown_tiles, format_pathname};

use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
};
use std::{path::PathBuf, usize};
use tiled_json_rs;

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
    let mut next_fire = 0.0;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let map_1 = tiled_json_rs::Map::load_from_file(&PathBuf::from("./maps/map_1.json").as_path())
        .expect("Failed to load map");

    let mut tiles_textures = vec![];
    for tileset in &map_1.tile_sets {
        let rl = &mut rl;
        let json_path = format_pathname(
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
    let hero_texture = &rl
        .load_texture(&thread, "./src/assets/img/Heroes/Rogue/Idle/Idle-Sheet.png")
        .expect("unable to load texture!");

    let hands_texture = &rl
        .load_texture(&thread, "./src/assets/img/Weapons/Hands/Hands.png")
        .expect("unable to load texture!");

    let weapon_texture = &rl
        .load_texture(&thread, "./src/assets/img/Weapons/Wood/Wood.png")
        .expect("unable to load texture!");

    let slash_texture = &rl
        .load_texture(&thread, "./src/assets/img/Anim/_Attack.png")
        .expect("unable to load texture!");

    let mut player_position = Vector2 {
        x: (20 * TILES_WIDTH) as f32,
        y: (30 * TILES_HEIGHT) as f32,
    };
    let frame_rec = Rectangle {
        x: 0.0,
        y: 0.0,
        width: (hero_texture.width / 4) as f32,
        height: (hero_texture.height) as f32,
    };
    let mut timer_current = 0.0;
    let timer_total = 1.0;
    let mut time_since_beginning = 1.0;

    rl.set_target_fps(60);

    while !&rl.window_should_close() {
        let frame_time = &rl.get_frame_time();
        timer_current += frame_time;
        if timer_current >= timer_total {
            timer_current -= timer_total;
            time_since_beginning += 1.0;
        }

        let is_attacking = &rl.is_key_down(KeyboardKey::KEY_Z);
        // player movement
        {
            let mut player_movement = Vector2 { x: 0.0, y: 0.0 };
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                player_movement.x = 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                player_movement.x = -1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                player_movement.y = 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_UP) {
                player_movement.y = -1.0;
            }

            let normalized_movement = &player_movement.normalized();
            let movement_speed = 64.0;
            player_position = Vector2 {
                x: player_position.x + (normalized_movement.x * movement_speed * frame_time),
                y: player_position.y + (normalized_movement.y * movement_speed * frame_time),
            };

            // Check player position to avoid moving outside tilemap limits
            if player_position.x < 0.0 {
                player_position.x = 0.0;
            }

            if player_position.y < 0.0 {
                player_position.y = 0.0;
            }

            if player_position.x >= (SCREEN_WIDTH as f32) - (hero_texture.width / 4) as f32 {
                player_position.x = (SCREEN_WIDTH as f32) - (hero_texture.width / 4) as f32;
            }

            if player_position.y >= (SCREEN_HEIGHT as f32) - (hero_texture.height) as f32 {
                player_position.y = (SCREEN_HEIGHT as f32) - (hero_texture.height) as f32;
            }
        }

        let d = &mut rl.begin_drawing(&thread);

        d.clear_background(Color::new(20, 20, 18, 1));
        // Drawing Tilemap
        // TODO: handle rotation
        {
            for (_layer_index, map_layer) in map_1.layers.iter().enumerate() {
                let mut x = 0;
                let mut y = 0;

                match &map_layer.layer_type {
                    tiled_json_rs::LayerType::TileLayer(tile_layer) => {
                        for tile_number_on_screen in 0..(TILE_WIDTH_COUNT * TILE_HEIGHT_COUNT) {
                            let tile_number_on_screen = tile_number_on_screen as usize;
                            let mut texture = &tiles_textures[0];
                            for (j, tiles) in map_1.tile_sets.iter().enumerate() {
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
                                    x: tile_arr[tile_layer.data[tile_number_on_screen] as usize].x,
                                    y: tile_arr[tile_layer.data[tile_number_on_screen] as usize].y,
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
                    tiled_json_rs::LayerType::ImageLayer(_image) => {
                        todo!()
                    }
                    tiled_json_rs::LayerType::ObjectGroup(_objects) => {
                        todo!()
                    }
                    tiled_json_rs::LayerType::Group { layers: _ } => {
                        todo!()
                    }
                }
            }
        }
        d.draw_texture_rec(hero_texture, frame_rec, player_position, Color::WHITE);
        d.draw_texture_rec(
            hands_texture,
            Rectangle {
                x: 0.0,
                y: 16.0,
                width: 32.0,
                height: 16.0,
            },
            Vector2 {
                x: player_position.x,
                y: player_position.y + 16.0,
            },
            Color::WHITE,
        );

        // simple attack anim
        let fire_rate = 1.5;
        if *is_attacking && time_since_beginning > next_fire {
            next_fire = time_since_beginning + fire_rate;
            d.draw_texture_rec(
                slash_texture,
                Rectangle {
                    x: 336.0,
                    y: 32.0,
                    height: (3 * 16) as f32,
                    width: (2 * 16) as f32,
                },
                Vector2 {
                    x: (player_position.x + 16.0),
                    y: (player_position.y - 16.0),
                },
                Color::WHITE,
            );
        } else {
            d.draw_texture_rec(
                weapon_texture,
                Rectangle {
                    x: 32.0,
                    y: 16.0,
                    height: 32.0,
                    width: 16.0,
                },
                Vector2 {
                    x: player_position.x + 16.0,
                    y: player_position.y,
                },
                Color::WHITE,
            );
        }
    }
}
