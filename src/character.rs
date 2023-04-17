use bevy::prelude::*;
use bevy_3d_sprite::{PbrSpriteBundle, SpriteAnimation};
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

const TAG_IDLE_DOWN: &'static str = "Idle_Down";
const TAG_IDLE_DOWN_RIGHT: &'static str = "Idle_DownRight";
const TAG_IDLE_UP: &'static str = "Idle_Up";
const TAG_IDLE_UP_RIGHT: &'static str = "Idle_UpRight";
const TAG_WALKING_DOWN: &'static str = "Walking_Down";
const TAG_WALKING_DOWN_RIGHT: &'static str = "Walking_DownRight";
const TAG_WALKING_UP: &'static str = "Walking_Up";
const TAG_WALKING_UP_RIGHT: &'static str = "Walking_UpRight";

const CHARACTER_FRAMES: [(&str, &[usize]); 8] = [
    (TAG_IDLE_DOWN, &[0, 1, 2, 3]),
    (TAG_IDLE_DOWN_RIGHT, &[4, 5, 6, 7]),
    (TAG_IDLE_UP, &[8, 9, 10, 11]),
    (TAG_IDLE_UP_RIGHT, &[12, 13, 14, 15]),
    (TAG_WALKING_DOWN, &[16, 17, 18, 19, 20, 21]),
    (TAG_WALKING_DOWN_RIGHT, &[22, 23, 24, 25, 26, 27]),
    (TAG_WALKING_UP, &[28, 29, 30, 31, 32, 33]),
    (TAG_WALKING_UP_RIGHT, &[34, 35, 36, 37, 38, 39]),
];

#[derive(Component)]
pub struct Character {
    direction: CharacterDirection,
    action: CharacterAction,
}

pub fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlases: ResMut<Assets<bevy_3d_sprite::TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    c.spawn(PbrSpriteBundle {
        texture_atlas: texture_atlases.add(bevy_3d_sprite::TextureAtlas::from_grid(
            asset_server.load("art/covict.png"),
            Vec2::new(24.0, 26.0),
            13,
            12,
        )),
        sprite: bevy_3d_sprite::TextureAtlasSprite {
            index: Some(0),
            flip_x: Some(false),
        },
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2.4, 2.6)))),
        material: materials.add(StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    })
    .insert(SpriteAnimation::from_loop(&CHARACTER_FRAMES, 0.1))
    .insert(Character {
        direction: CharacterDirection::Down,
        action: CharacterAction::Idle,
    })
    .insert(Name::new("Character"));
}

pub fn update_character_sprite(
    mut query: Query<(
        &mut bevy_3d_sprite::TextureAtlasSprite,
        &mut SpriteAnimation,
        &Character,
    )>,
) {
    for (mut sprite, mut anima, char) in query.iter_mut() {
        match char.action {
            CharacterAction::Idle => match char.direction {
                CharacterDirection::Down => {
                    anima.update(TAG_IDLE_DOWN);
                }
                CharacterDirection::DownLeft => {
                    anima.update(TAG_IDLE_DOWN_RIGHT);
                    sprite.flip_x = Some(true);
                }
                CharacterDirection::DownRight => {
                    anima.update(TAG_IDLE_DOWN_RIGHT);
                    sprite.flip_x = Some(false);
                }
                CharacterDirection::Up => {
                    anima.update(TAG_IDLE_UP);
                }
                CharacterDirection::UpLeft => {
                    anima.update(TAG_IDLE_UP_RIGHT);
                    sprite.flip_x = Some(true);
                }
                CharacterDirection::UpRight => {
                    anima.update(TAG_IDLE_UP_RIGHT);
                    sprite.flip_x = Some(false);
                }
            },
            CharacterAction::Walking => match char.direction {
                CharacterDirection::Down => {
                    anima.update(TAG_WALKING_DOWN);
                }
                CharacterDirection::DownLeft => {
                    anima.update(TAG_WALKING_DOWN_RIGHT);
                    sprite.flip_x = Some(true);
                }
                CharacterDirection::DownRight => {
                    anima.update(TAG_WALKING_DOWN_RIGHT);
                    sprite.flip_x = Some(false);
                }
                CharacterDirection::Up => {
                    anima.update(TAG_WALKING_UP);
                }
                CharacterDirection::UpLeft => {
                    anima.update(TAG_WALKING_UP_RIGHT);
                    sprite.flip_x = Some(true);
                }
                CharacterDirection::UpRight => {
                    anima.update(TAG_WALKING_UP_RIGHT);
                    sprite.flip_x = Some(false);
                }
            },
        }
    }
}

pub fn character_move(
    mut query: Query<(&mut Transform, &mut Character)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let speed = 0.07_f32;
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

        if walking {
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
    query: Query<&bevy_3d_sprite::TextureAtlasSprite, With<Character>>,
    asset_server: Res<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
) {
    if !audio.is_playing_sound() {
        for sprite in query.iter() {
            if let Some(index) = sprite.index {
                match index {
                    17 | 20 | 23 | 26 | 29 | 32 | 35 | 38 => {
                        match rand::thread_rng().gen_range(1..4) {
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
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
