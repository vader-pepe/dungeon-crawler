mod map_helper;
use map_helper::{breakdown_tiles, format_pathname, parse_json_to_map};

use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
};

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

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let map_1 = parse_json_to_map("./maps/map_1.json");
    let mut tiles_textures = vec![];
    for (_, tileset) in map_1.tilesets.iter().enumerate() {
        let rl = &mut rl;
        let json_path = format_pathname(tileset.image.as_str());
        let t = rl
            .load_texture(&thread, json_path.as_str())
            .expect("cannot load texture!");
        tiles_textures.push(t);
    }

    let tile_arr = breakdown_tiles(&map_1.tilesets);

    // TODO: handle player texture better
    let hero_texture = &rl
        .load_texture(&thread, "./src/assets/img/Heroes/Rogue/Idle/Idle-Sheet.png")
        .expect("unable to load texture");

    let mut player_position = Vector2 { x: 0.0, y: 0.0 };
    let frame_rec = Rectangle {
        x: 0.0,
        y: 0.0,
        width: (hero_texture.width / 4) as f32,
        height: (hero_texture.height) as f32,
    };
    let mut current_frame = 0;
    let mut frame_counter = 0;
    let frame_speed = 8;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        frame_counter += 1;

        if frame_counter >= (60 / frame_speed) {
            frame_counter = 0;
            current_frame += 1;

            if current_frame > 5 {
                current_frame = 0;
            }
        }

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

            let normalized_movement = player_movement.normalized();
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

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(20, 20, 18, 1));

        // Drawing Tilemap
        {
            for (_layer_index, layer) in map_1.layers.iter().enumerate() {
                let data = &layer.data;
                let mut x = 0;
                let mut y = 0;

                if layer.visible {
                    for i in 0..(TILE_WIDTH_COUNT * TILE_HEIGHT_COUNT) {
                        let i = i as usize;
                        let mut texture = &tiles_textures[0];
                        for (j, tiles) in map_1.tilesets.iter().enumerate() {
                            if layer.data[i] >= tiles.firstgid
                                && layer.data[i] <= (tiles.firstgid + tiles.tilecount - 1)
                            {
                                texture = &tiles_textures[j]
                            }
                        }
                        d.draw_texture_rec(
                            texture,
                            Rectangle {
                                x: tile_arr[data[i] as usize].x,
                                y: tile_arr[data[i] as usize].y,
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
            }
        }
        d.draw_texture_rec(hero_texture, frame_rec, player_position, Color::WHITE);
    }
}
