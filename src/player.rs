use raylib::prelude::RaylibDraw;
use raylib::RaylibHandle;
use raylib::{
    color::Color,
    drawing::RaylibDrawHandle,
    ffi::KeyboardKey,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

#[derive(Debug)]
pub enum PlayerState {
    Idle,
    Attack,
    Walk,
}

#[derive(Debug)]
enum PlayerFacing {
    Left,
    Right,
}

#[derive(Debug)]
pub struct PlayerTextures<'a> {
    pub torso: Vec<&'a Texture2D>,
    pub hands: &'a Texture2D,
    pub slash: &'a Texture2D,
    pub weapon: &'a Texture2D,
}

#[derive(Debug)]
pub struct Frames<'a> {
    pub idle_frame: &'a mut f32,
    pub attack_frame: &'a mut f32,
    pub walk_frame: &'a mut f32,
}

#[derive(Debug)]
pub struct Player<'a> {
    state: PlayerState,
    position: &'a mut Vector2,
    textures: &'a mut PlayerTextures<'a>,
    frame_speed: f32,
    frames: &'a mut Frames<'a>,
    movement_speed: f32,
    facing: PlayerFacing,
}

impl<'a> Player<'a> {
    pub fn new(
        textures: &'a mut PlayerTextures<'a>,
        frames: &'a mut Frames<'a>,
        initial_pos: &'a mut Vector2,
    ) -> Self {
        Self {
            state: PlayerState::Idle,
            frame_speed: 0.0,
            position: initial_pos,
            movement_speed: 64.0,
            facing: PlayerFacing::Right,
            frames,
            textures,
        }
    }

    pub fn process(&mut self, rl: &RaylibHandle) {
        let frame_time = &rl.get_frame_time();
        let mut movement = Vector2 { x: 0.0, y: 0.0 };
        self.frame_speed = frame_time * 3.0;
        self.state = PlayerState::Idle;

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.state = PlayerState::Walk;
            movement.y -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.state = PlayerState::Walk;
            self.facing = PlayerFacing::Right;
            movement.x += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.state = PlayerState::Walk;
            movement.y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.state = PlayerState::Walk;
            self.facing = PlayerFacing::Left;
            movement.x -= 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_Z) {
            self.state = PlayerState::Attack;
        }

        let normalized_movement = &movement.normalized();
        self.position.x =
            self.position.x + (normalized_movement.x * self.movement_speed * frame_time);
        self.position.y =
            self.position.y + (normalized_movement.y * self.movement_speed * frame_time);

        match self.state {
            PlayerState::Idle => {
                *self.frames.idle_frame += 1.0 * self.frame_speed;
                if *self.frames.idle_frame >= 4.0 {
                    *self.frames.idle_frame = 0.0;
                }
            }
            PlayerState::Attack => {
                *self.frames.attack_frame += 1.0 * self.frame_speed;
                if *self.frames.attack_frame >= 5.5 {
                    *self.frames.attack_frame = 0.0;
                    self.state = PlayerState::Idle;
                }
            }
            PlayerState::Walk => {
                *self.frames.walk_frame += 1.0 * self.frame_speed * 2.0;
                if *self.frames.walk_frame >= 6.0 {
                    *self.frames.walk_frame = 0.0;
                }
            }
        }
    }
}

pub fn render_player(d: &mut RaylibDrawHandle, player: &mut Player) {
    match player.state {
        PlayerState::Idle => {
            let texture_width = match player.facing {
                PlayerFacing::Left => -(player.textures.torso[0].width / 4),
                PlayerFacing::Right => player.textures.torso[0].width / 4,
            };
            d.draw_texture_rec(
                player.textures.torso[0],
                Rectangle {
                    x: (player.frames.idle_frame.floor() * 32.0),
                    y: 0.0,
                    width: texture_width as f32,
                    height: (player.textures.torso[0].height) as f32,
                },
                *player.position,
                Color::WHITE,
            );
            d.draw_texture_rec(
                player.textures.hands,
                Rectangle {
                    x: 0.0,
                    y: 16.0,
                    width: 32.0,
                    height: 16.0,
                },
                Vector2 {
                    x: player.position.x,
                    y: player.position.y + 16.0,
                },
                Color::WHITE,
            );
            d.draw_texture_rec(
                player.textures.weapon,
                Rectangle {
                    x: 32.0,
                    y: 16.0,
                    height: 32.0,
                    width: 16.0,
                },
                Vector2 {
                    x: player.position.x + 16.0,
                    y: player.position.y,
                },
                Color::WHITE,
            );
        }
        PlayerState::Walk => {
            let texture_width = match player.facing {
                PlayerFacing::Left => -(player.textures.torso[1].width / 6),
                PlayerFacing::Right => player.textures.torso[1].width / 6,
            };

            d.draw_texture_rec(
                player.textures.torso[1],
                Rectangle {
                    x: (player.frames.walk_frame.floor() * 32.0),
                    y: 0.0,
                    width: texture_width as f32,
                    height: (player.textures.torso[1].height) as f32,
                },
                *player.position,
                Color::WHITE,
            );
            d.draw_texture_rec(
                player.textures.hands,
                Rectangle {
                    x: 0.0,
                    y: 16.0,
                    width: 32.0,
                    height: 16.0,
                },
                Vector2 {
                    x: player.position.x,
                    y: player.position.y + 16.0,
                },
                Color::WHITE,
            );
            d.draw_texture_rec(
                player.textures.weapon,
                Rectangle {
                    x: 32.0,
                    y: 16.0,
                    height: 32.0,
                    width: 16.0,
                },
                Vector2 {
                    x: player.position.x + 16.0,
                    y: player.position.y,
                },
                Color::WHITE,
            );
        }
        PlayerState::Attack => {
            let texture_width = match player.facing {
                PlayerFacing::Left => -(player.textures.torso[0].width / 4),
                PlayerFacing::Right => player.textures.torso[0].width / 4,
            };
            d.draw_texture_rec(
                player.textures.torso[0],
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture_width as f32,
                    height: (player.textures.torso[0].height) as f32,
                },
                *player.position,
                Color::WHITE,
            );
            d.draw_texture_rec(
                player.textures.hands,
                Rectangle {
                    x: 0.0,
                    y: 16.0,
                    width: 32.0,
                    height: 16.0,
                },
                Vector2 {
                    x: player.position.x,
                    y: player.position.y + 16.0,
                },
                Color::WHITE,
            );
            d.draw_texture_rec(
                player.textures.slash,
                Rectangle {
                    x: 336.0,
                    y: 32.0,
                    height: (3 * 16) as f32,
                    width: (2 * 16) as f32,
                },
                Vector2 {
                    x: (player.position.x + 16.0),
                    y: (player.position.y - 16.0),
                },
                Color::WHITE,
            );
        }
    }
}
