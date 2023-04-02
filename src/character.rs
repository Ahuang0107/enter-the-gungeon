use crate::sprite_animation::SpriteAnimation;
use crate::tilemap::CollisionInfo;
use bevy::prelude::*;
use bevy_kira_audio::AudioControl;
use rand::Rng;

#[derive(PartialEq)]
pub enum CharacterAction {
    Idle,
    Walking,
}

pub enum CharacterDirection {
    Down,
    DownLeft,
    DownRight,
    Up,
    UpLeft,
    UpRight,
}

#[derive(Component)]
pub struct Character {
    direction: CharacterDirection,
    action: CharacterAction,
}

pub fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_altlas = TextureAtlas::from_grid(
        asset_server.load("art/covict.png"),
        Vec2::new(24.0, 26.0),
        13,
        12,
        None,
        None,
    );
    c.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_altlas),
        transform: Transform {
            translation: Vec3::new(64.0, -64.0, 10.0),
            ..default()
        },
        ..default()
    })
    .insert(SpriteAnimation::from_loop(
        &[
            ("Idle_Down", &[0, 1, 2, 3]),
            ("Idle_DownRight", &[4, 5, 6, 7]),
            ("Idle_Up", &[8, 9, 10, 11]),
            ("Idle_UpRight", &[12, 13, 14, 15]),
            ("Walking_Down", &[16, 17, 18, 19, 20, 21]),
            ("Walking_DownRight", &[22, 23, 24, 25, 26, 27]),
            ("Walking_Up", &[28, 29, 30, 31, 32, 33]),
            ("Walking_UpRight", &[34, 35, 36, 37, 38, 39]),
        ],
        0.1,
    ))
    .insert(Character {
        direction: CharacterDirection::Down,
        action: CharacterAction::Idle,
    });
}

pub fn update_character_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &mut SpriteAnimation, &Character)>,
) {
    for (mut sprite, mut anima, char) in query.iter_mut() {
        match char.action {
            CharacterAction::Idle => match char.direction {
                CharacterDirection::Down => {
                    anima.update("Idle_Down");
                }
                CharacterDirection::DownLeft => {
                    anima.update("Idle_DownRight");
                    sprite.flip_x = true;
                }
                CharacterDirection::DownRight => {
                    anima.update("Idle_DownRight");
                    sprite.flip_x = false;
                }
                CharacterDirection::Up => {
                    anima.update("Idle_Up");
                }
                CharacterDirection::UpLeft => {
                    anima.update("Idle_UpRight");
                    sprite.flip_x = true;
                }
                CharacterDirection::UpRight => {
                    anima.update("Idle_UpRight");
                    sprite.flip_x = false;
                }
            },
            CharacterAction::Walking => match char.direction {
                CharacterDirection::Down => {
                    anima.update("Walking_Down");
                }
                CharacterDirection::DownLeft => {
                    anima.update("Walking_DownRight");
                    sprite.flip_x = true;
                }
                CharacterDirection::DownRight => {
                    anima.update("Walking_DownRight");
                    sprite.flip_x = false;
                }
                CharacterDirection::Up => {
                    anima.update("Walking_Up");
                }
                CharacterDirection::UpLeft => {
                    anima.update("Walking_UpRight");
                    sprite.flip_x = true;
                }
                CharacterDirection::UpRight => {
                    anima.update("Walking_UpRight");
                    sprite.flip_x = false;
                }
            },
        }
    }
}

pub fn character_move(
    mut query: Query<(&mut Transform, &mut Character)>,
    keyboard: Res<Input<KeyCode>>,
    collision_info: Res<CollisionInfo>,
) {
    let speed = 0.7_f32;
    const RATIO: f32 = std::f32::consts::SQRT_2 / 2.0;

    for (mut t, mut char) in query.iter_mut() {
        let mut direction: Option<CharacterDirection> = None;
        let mut walking = false;
        let mut old = t.translation;

        if keyboard.pressed(KeyCode::W) {
            if keyboard.pressed(KeyCode::A) {
                old.y += speed * RATIO;
                old.x -= speed * RATIO;
                direction = Some(CharacterDirection::UpLeft);
            } else if keyboard.pressed(KeyCode::D) {
                old.y += speed * RATIO;
                old.x += speed * RATIO;
                direction = Some(CharacterDirection::UpRight);
            } else {
                old.y += speed;
                direction = Some(CharacterDirection::Up);
            }
            walking = true;
        } else if keyboard.pressed(KeyCode::S) {
            if keyboard.pressed(KeyCode::A) {
                old.y -= speed * RATIO;
                old.x -= speed * RATIO;
                direction = Some(CharacterDirection::DownLeft);
            } else if keyboard.pressed(KeyCode::D) {
                old.y -= speed * RATIO;
                old.x += speed * RATIO;
                direction = Some(CharacterDirection::DownRight);
            } else {
                old.y -= speed;
                direction = Some(CharacterDirection::Down);
            }
            walking = true;
        } else if keyboard.pressed(KeyCode::A) {
            old.x -= speed;
            direction = Some(CharacterDirection::DownLeft);
            walking = true;
        } else if keyboard.pressed(KeyCode::D) {
            old.x += speed;
            direction = Some(CharacterDirection::DownRight);
            walking = true;
        }

        let c_x = (old.x / 16.0) as usize;
        let c_y = (-old.y / 16.0) as usize;
        if collision_info.tiles[c_y][c_x] > 0 {
            t.translation = old;
        }

        if let Some(direction) = direction {
            char.direction = direction;
        }

        if walking {
            char.action = CharacterAction::Walking;
        } else {
            char.action = CharacterAction::Idle;
        }
    }
}

pub fn play_character_sound(
    query: Query<&TextureAtlasSprite, With<Character>>,
    asset_server: Res<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
) {
    if !audio.is_playing_sound() {
        for sprite in query.iter() {
            match sprite.index {
                17 | 20 | 23 | 26 | 29 | 32 | 35 | 38 => match rand::thread_rng().gen_range(1..4) {
                    1 => {
                        audio.play(asset_server.load("sound/barefoot_stone_01.wav"));
                    }
                    2 => {
                        audio.play(asset_server.load("sound/barefoot_stone_02.wav"));
                    }
                    3 => {
                        audio.play(asset_server.load("sound/barefoot_stone_03.wav"));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
