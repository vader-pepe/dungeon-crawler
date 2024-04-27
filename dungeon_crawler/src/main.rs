use map_loader::{parse_json, JSONType};
use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
};
mod map_loader;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
// tile size fixed 16x16 pixel
const TILES_WIDTH: i32 = 16;
const TILES_HEIGHT: i32 = 16;
// total tiles horizontally is 40 because
// 16x40 = 640
const TILE_WIDTH_COUNT: i32 = 40;
// total tiles vertically is 30 because
// 16x30 = 480
const TILE_HEIGHT_COUNT: i32 = 30;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let tiles = parse_json(JSONType::Tiles, "Tiles.json").unwrap_left();
    let map = parse_json(JSONType::Map, "dungeon-crawler.json").unwrap_right();
    let map_layers = map.layers.clone();
    // determining tile id to be drawn to the screen
    let mut tile_to_draw: Vec<usize> = vec![]; // this has to be out of the expression
    {
        for (_, layer) in map_layers.iter().enumerate() {
            // TODO: here layers not yet used ^^^
            for (_, value) in layer.data.iter().enumerate() {
                let x = *value;
                tile_to_draw.push(x as usize);
            }
        }
    }

    let tiles_texture = &rl
        .load_texture(&thread, tiles.image.as_str())
        .expect("unable to load texture");
    // first element must be empty
    let mut tile_arr: Vec<Rectangle> = vec![Rectangle {
        x: -16.0,
        y: -16.0,
        width: 0.0,
        height: 0.0,
    }];

    // tiles breakdown
    {
        let mut temp_x = 0;
        let mut temp_y = 0;
        for _ in 0..tiles.tilecount {
            tile_arr.push(Rectangle {
                x: temp_x as f32,
                y: temp_y as f32,
                width: TILES_WIDTH as f32,
                height: TILES_HEIGHT as f32,
            });
            temp_x += TILES_WIDTH;
            if temp_x % tiles.columns == 0 {
                temp_x = 0;
                temp_y += TILES_HEIGHT;
            }
        }
    }

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
            let movement_speed = 32.0;
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
            let mut i: usize = 0;
            let mut temp_x = 0;
            let mut temp_y = 0;
            // TODO: draw separate layers

            for _ in 0..(TILE_WIDTH_COUNT * TILE_HEIGHT_COUNT) {
                d.draw_texture_rec(
                    tiles_texture,
                    Rectangle {
                        x: tile_arr[tile_to_draw[i]].x,
                        y: tile_arr[tile_to_draw[i]].y,
                        width: TILES_WIDTH as f32,
                        height: TILES_HEIGHT as f32,
                    },
                    Vector2 {
                        x: temp_x as f32,
                        y: temp_y as f32,
                    },
                    Color::WHITE,
                );
                temp_x += TILES_WIDTH;
                if temp_x % SCREEN_WIDTH == 0 {
                    temp_x = 0;
                    temp_y += TILES_HEIGHT;
                }
                i += 1;
            }
        }
        d.draw_texture_rec(hero_texture, frame_rec, player_position, Color::WHITE);
    }
}
