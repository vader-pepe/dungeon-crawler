use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
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
    let mut tile_boxes: Vec<Rectangle> = vec![Rectangle {
        x: -1.0,
        y: -1.0,
        width: 0.0,
        height: 0.0,
    }];

    for x in 0..tile_texture.width {
        for y in 0..tile_texture.height {
            if x % 16 == 0 && y % 16 == 0 {
                tile_boxes.push(Rectangle {
                    x: (x as f32),
                    y: (y as f32),
                    width: 16.0,
                    height: 16.0,
                });
            }
        }
    }

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
        let frame_time = rl.get_frame_time();
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

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(20, 20, 18, 1));
        let starting_coordinate = Vector2 { x: 128.0, y: 96.0 };
        let end_coordinate = Vector2 {
            x: (128.0 + (17.0 * 16.0)),
            y: (96.0 + (10.0 * 16.0)),
        };
        // TODO: create a tile mapper rather than this shit
        // UPDATE: use tiled, export to json, parse here!
        let tile_to_draw = vec![
            80, 81, 81, 81, 81, 81, 82, 0, 0, 0, // first 10 column
            58, 59, 60, 101, 102, 103, 29, 0, 0, 0, // 20
            83, 84, 85, 126, 127, 128, 29, 0, 0, 0, // 30
            1, 2, 3, 126, 127, 128, 29, 0, 0, 0, // 40
            26, 27, 28, 126, 127, 128, 29, 0, 0, 0, // 50
            51, 52, 53, 213, 214, 128, 29, 0, 0, 0, // 60
            40, 41, 42, 43, 239, 128, 54, 81, 81, 82, // 70
            26, 27, 28, 20, 21, 22, 104, 134, 135, 29, // 80
            26, 27, 44, 45, 46, 47, 129, 159, 160, 29, // 90
            51, 52, 53, 70, 71, 72, 129, 159, 160, 29, // 100
            15, 16, 17, 18, 127, 128, 4, 106, 106, 107, // 110
            30, 101, 102, 102, 127, 128, 29, 0, 0, 0, // 120
            26, 126, 127, 127, 127, 128, 29, 0, 0, 0, // 130
            26, 126, 127, 127, 152, 153, 29, 0, 0, 0, // 140
            26, 126, 127, 128, 182, 183, 29, 0, 0, 0, // 150
            26, 126, 152, 153, 207, 208, 29, 0, 0, 0, // 160
            105, 106, 106, 106, 106, 106, 107, 0, 0, 0, // 170
        ];

        let mut i = 0;
        // Drawing Tilemap
        for x in (starting_coordinate.x as i32)..(end_coordinate.x as i32) {
            for y in (starting_coordinate.y as i32)..(end_coordinate.y as i32) {
                if x % 16 == 0 && y % 16 == 0 {
                    d.draw_texture_rec(
                        tile_texture,
                        Rectangle {
                            x: (tile_boxes[tile_to_draw[i]].x as f32),
                            y: (tile_boxes[tile_to_draw[i]].y as f32),
                            width: (tile_boxes[tile_to_draw[i]].width as f32),
                            height: (tile_boxes[tile_to_draw[i]].height as f32),
                        },
                        Vector2 {
                            x: (x as f32),
                            y: (y as f32),
                        },
                        Color::WHITE,
                    );
                    i += 1;
                }
            }
        }

        d.draw_texture_rec(hero_texture, frame_rec, player_position, Color::WHITE);
    }
}
