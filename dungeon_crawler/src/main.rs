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
    let mut tile_boxes: Vec<Rectangle> = vec![];

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

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            player_position.x += 1 as f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            player_position.x -= 1 as f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            player_position.y += 1 as f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            player_position.y -= 1 as f32;
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

        d.clear_background(Color::new(20, 20, 18, 1));
        let starting_coordinate = Vector2 { x: 128.0, y: 96.0 };
        let end_coordinate = Vector2 {
            x: (128.0 + (17.0 * 16.0)),
            y: (96.0 + (10.0 * 16.0)),
        };
        // TODO: create a tile mapper rather than this shit
        let tile_to_draw = vec![
            79, 80, 80, 80, 80, 80, 81, 311, 311, 311, // first 10 column
            57, 58, 59, 100, 101, 102, 28, 311, 311, 311, // 20
            82, 83, 84, 125, 126, 127, 28, 311, 311, 311, // 30
            0, 1, 2, 125, 126, 127, 28, 311, 311, 311, // 40
            25, 26, 27, 125, 126, 127, 28, 311, 311, 311, // 50
            50, 51, 52, 212, 213, 127, 28, 311, 311, 311, // 60
            39, 40, 41, 42, 238, 127, 53, 80, 80, 81, // 70
            25, 26, 27, 19, 20, 21, 103, 133, 134, 28, // 80
            25, 26, 43, 44, 45, 46, 128, 158, 159, 28, // 90
            50, 51, 52, 69, 70, 71, 128, 158, 159, 28, // 100
            14, 15, 16, 17, 126, 127, 3, 105, 105, 106, // 110
            29, 100, 101, 101, 126, 127, 28, 311, 311, 311, // 120
            25, 125, 126, 126, 126, 127, 28, 311, 311, 311, // 130
            25, 125, 126, 126, 151, 152, 28, 311, 311, 311, // 140
            25, 125, 126, 127, 181, 182, 28, 311, 311, 311, // 150
            25, 125, 151, 152, 206, 207, 28, 311, 311, 311, // 160
            104, 105, 105, 105, 105, 105, 106, 311, 311, 311, // 170
        ];

        let mut i = 0;
        for x in (starting_coordinate.x as i32)..(end_coordinate.x as i32) {
            for y in (starting_coordinate.y as i32)..(end_coordinate.y as i32) {
                if x % 16 == 0 && y % 16 == 0 {
                    d.draw_texture_rec(
                        tile_texture,
                        Rectangle {
                            x: (tile_boxes[tile_to_draw[i]].x as f32),
                            y: (tile_boxes[tile_to_draw[i]].y as f32),
                            width: (tile_boxes[0].width as f32),
                            height: (tile_boxes[0].height as f32),
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
