use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::{KeyboardKey, Rectangle, Vector2},
};

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let tile_texture = &rl
        .load_texture(
            &thread,
            "./src/assets/img/Environment/Dungeon Prison/Assets/Tiles.png",
        )
        .expect("unable to load texture");

    let hero_texture = &rl
        .load_texture(&thread, "./src/assets/img/Heroes/Rogue/Idle/Idle-Sheet.png")
        .expect("unable to load texture");
    let map_width: i32 = 16;
    let map_height: i32 = 16;
    let mut tile_dest: Rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 16.0,
        height: 16.0,
    };
    let mut tile_src: Rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 16.0,
        height: 16.0,
    };
    let mut tile_map: Vec<i32> = vec![0];
    tile_map.push(99);
    let mut src_map: Vec<&str>;

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

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // TODO: fix this abomination
        for (i, val) in tile_map.iter().enumerate() {
            if *val != 0 {
                tile_dest.x = tile_dest.width * ((i as f32) % (map_width as f32));
                tile_dest.y = tile_dest.height * ((i as f32) % (map_height as f32));
                tile_src.x = tile_src.width
                    * (((*val as f32) - 1.0) % ((tile_texture.width as f32) / tile_src.width));
                tile_src.y = tile_src.height
                    * (((*val as f32) - 1.0) / ((tile_texture.width as f32) / tile_src.width))
            }
        }

        d.draw_texture_rec(
            tile_texture,
            tile_dest,
            Vector2 { x: 0.0, y: 0.0 },
            Color::WHITE,
        );
        d.draw_texture_rec(hero_texture, frame_rec, player_position, Color::WHITE);
    }
}
