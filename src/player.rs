use raylib::prelude::RaylibDraw;
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
pub struct Player {
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::Idle,
        }
    }

    pub fn process(&mut self, key: Option<KeyboardKey>) {
        match key {
            Some(KeyboardKey::KEY_Z) => {
                self.state = PlayerState::Attack;
            }
            Some(KeyboardKey::KEY_UP) => self.state = PlayerState::Walk,
            Some(KeyboardKey::KEY_DOWN) => self.state = PlayerState::Walk,
            Some(KeyboardKey::KEY_RIGHT) => self.state = PlayerState::Walk,
            Some(KeyboardKey::KEY_LEFT) => self.state = PlayerState::Walk,
            _ => self.state = PlayerState::Idle,
        }
    }
}

pub fn render_player(
    d: &mut RaylibDrawHandle,
    player_torso_texture: &Texture2D,
    player_hands_texture: &Texture2D,
    slash_texture: &Texture2D,
    weapon_texture: &Texture2D,
    position: Vector2,
    state: &PlayerState,
    current_player_frame: &mut f32,
    frame_speed: &f32,
) {
    match state {
        PlayerState::Idle => {
            d.draw_texture_rec(
                player_torso_texture,
                Rectangle {
                    x: (current_player_frame.floor() * 32.0),
                    y: 0.0,
                    width: (player_torso_texture.width / 4) as f32,
                    height: (player_torso_texture.height) as f32,
                },
                position,
                Color::WHITE,
            );
            d.draw_texture_rec(
                player_hands_texture,
                Rectangle {
                    x: 0.0,
                    y: 16.0,
                    width: 32.0,
                    height: 16.0,
                },
                Vector2 {
                    x: position.x,
                    y: position.y + 16.0,
                },
                Color::WHITE,
            );
            d.draw_texture_rec(
                weapon_texture,
                Rectangle {
                    x: 32.0,
                    y: 16.0,
                    height: 32.0,
                    width: 16.0,
                },
                Vector2 {
                    x: position.x + 16.0,
                    y: position.y,
                },
                Color::WHITE,
            );
        }
        PlayerState::Walk => {
            d.draw_texture_rec(
                player_torso_texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: (player_torso_texture.width / 4) as f32,
                    height: (player_torso_texture.height) as f32,
                },
                position,
                Color::WHITE,
            );
            d.draw_texture_rec(
                player_hands_texture,
                Rectangle {
                    x: 0.0,
                    y: 16.0,
                    width: 32.0,
                    height: 16.0,
                },
                Vector2 {
                    x: position.x,
                    y: position.y + 16.0,
                },
                Color::WHITE,
            );
            d.draw_texture_rec(
                weapon_texture,
                Rectangle {
                    x: 32.0,
                    y: 16.0,
                    height: 32.0,
                    width: 16.0,
                },
                Vector2 {
                    x: position.x + 16.0,
                    y: position.y,
                },
                Color::WHITE,
            );
        }
        PlayerState::Attack => {
            d.draw_texture_rec(
                player_torso_texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: (player_torso_texture.width / 4) as f32,
                    height: (player_torso_texture.height) as f32,
                },
                position,
                Color::WHITE,
            );
            d.draw_texture_rec(
                player_hands_texture,
                Rectangle {
                    x: 0.0,
                    y: 16.0,
                    width: 32.0,
                    height: 16.0,
                },
                Vector2 {
                    x: position.x,
                    y: position.y + 16.0,
                },
                Color::WHITE,
            );
            d.draw_texture_rec(
                slash_texture,
                Rectangle {
                    x: 336.0,
                    y: 32.0,
                    height: (3 * 16) as f32,
                    width: (2 * 16) as f32,
                },
                Vector2 {
                    x: (position.x + 16.0),
                    y: (position.y - 16.0),
                },
                Color::WHITE,
            );
        }
    }

    *current_player_frame += 1.0 * frame_speed;
    if *current_player_frame >= 4.0 {
        *current_player_frame = 0.0;
    }
}
