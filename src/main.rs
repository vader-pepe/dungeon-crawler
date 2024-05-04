mod tiles_helper;
use tiles_helper::{breakdown_tiles, format_pathname};

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};
use std::path::PathBuf;
use tiled_json_rs::{Map, ObjectType};

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

    let mut tiles_textures: Vec<Texture2D> = vec![];
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
    // tips: create prefabs(game object?) like unity
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

    let door_rec = Rectangle {
        x: (19 * TILES_WIDTH) as f32,
        y: (22 * TILES_HEIGHT) as f32,
        width: (2 * TILES_WIDTH) as f32,
        height: (1 * TILES_HEIGHT) as f32,
    };

    let mut _player_hitbox = Rectangle {
        x: player_position.x,
        y: player_position.y,
        width: (hero_texture.width / 4) as f32,
        height: hero_texture.height as f32,
    };

    let mut timer_current = 0.0;
    let timer_total = 1.0;

    rl.set_target_fps(60);

    while !&rl.window_should_close() {
        let frame_time = &rl.get_frame_time();
        let time_since_beginning = &rl.get_time();
        timer_current += frame_time;
        if timer_current >= timer_total {
            timer_current -= timer_total;
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

            _player_hitbox = Rectangle {
                x: player_position.x,
                y: player_position.y,
                width: (hero_texture.width / 4) as f32,
                height: (hero_texture.height / 2) as f32,
            };

            // these 2 are basically the same
            let check_collision = _player_hitbox.check_collision_recs(&door_rec); // returning
                                                                                  // boolean
            let get_collision = _player_hitbox.get_collision_rec(&door_rec); // returnin Rectangle
            match get_collision {
                None => (),
                Some(_rect) => { /* TODO: handle collision,*/ }
            };

            if check_collision {
                // TODO: handle collision
            }

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

        draw_scene(d, &map_1, &tiles_textures, &tile_arr);

        d.draw_texture_rec(
            hero_texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: (hero_texture.width / 4) as f32,
                height: (hero_texture.height) as f32,
            },
            player_position,
            Color::WHITE,
        );
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
        let fire_rate = 0.5;
        if *is_attacking && time_since_beginning > &next_fire {
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

        // d.draw_triangle(
        //     Vector2 { x: 16.0, y: 16.0 },
        //     Vector2 { x: 16.0, y: 32.0 },
        //     Vector2 { x: 32.0, y: 32.0 },
        //     Color::RED,
        // );
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
                    for (obj_index, object) in obj_group.objects.iter().enumerate() {
                        match &object.object_type {
                            ObjectType::None => {
                                d.draw_rectangle(
                                    object.x as i32,
                                    object.y as i32,
                                    object.width as i32,
                                    object.height as i32,
                                    Color::RED,
                                );
                            }
                            _ => (),
                        };
                    }
                }
                _ => (),
            }
        }
    }
}
