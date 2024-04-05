use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dungeon Crawler")
        .build();

    let hero_texture = &rl
            .load_texture(&thread, "/home/ihsankl/Documents/github/dungeon-crawler/dungeon_crawler/src/assets/img/Heroes/Rogue/Idle/Idle-Sheet.png")
            .unwrap();

    let vector2d = vec![0, 0];
    let frame_rec = vec![0, 0, hero_texture.width / 4, hero_texture.height];
    let mut current_frame = 0;
    let mut frame_counter = 0;
    let mut frame_speed = 8;

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
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_texture(hero_texture, 0, 0, Color::WHITE);
    }
}
