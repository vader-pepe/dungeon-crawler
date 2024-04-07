use raylib::{ffi::Vector2, prelude::*};

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let hero_texture = &rl
        .load_texture(&thread, "./src/assets/img/Heroes/Rogue/Idle/Idle-Sheet.png")
        .unwrap();

    let mut player_position = Vector2 { x: 0.0, y: 0.0 };
    let mut player_tile_x = 0;
    let mut player_tile_y = 0;
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
        frame_counter += 1;

        if frame_counter >= (60 / frame_speed) {
            frame_counter = 0;
            current_frame += 1;

            if current_frame > 5 {
                current_frame = 0;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            player_position.x += 5 as f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            player_position.x -= 5 as f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            player_position.y += 5 as f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            player_position.y -= 5 as f32;
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_texture_rec(hero_texture, frame_rec, player_position, Color::WHITE);
    }
}
