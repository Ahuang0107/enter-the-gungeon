use std::f32::consts::SQRT_2;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_kira_audio::AudioControl;
use rand::Rng;

use crate::resource::{ResourceCache, GRID_SIZE, SCALE_RATIO};
use crate::sprite_animation::{MaterialSprite, SpriteAnimation};

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

impl CharacterDirection {
    pub fn to_up(&mut self) {
        *self = Self::Up
    }
    pub fn to_down(&mut self) {
        *self = Self::Down
    }
    pub fn to_left(&mut self) {
        *self = Self::DownLeft
    }
    pub fn to_right(&mut self) {
        *self = Self::DownRight
    }
}

const TAG_IDLE_DOWN: &'static str = "Idle_Down";
const TAG_IDLE_DOWN_RIGHT: &'static str = "Idle_DownRight";
const TAG_IDLE_UP: &'static str = "Idle_Up";
const TAG_IDLE_UP_RIGHT: &'static str = "Idle_UpRight";
const TAG_WALKING_DOWN: &'static str = "Walking_Down";
const TAG_WALKING_DOWN_RIGHT: &'static str = "Walking_DownRight";
const TAG_WALKING_UP: &'static str = "Walking_Up";
const TAG_WALKING_UP_RIGHT: &'static str = "Walking_UpRight";

const CHARACTER_FRAMES: [(&str, &[u8]); 8] = [
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
    move_speed: f32,
}

pub fn setup(mut c: Commands, cache: Res<ResourceCache>) {
    let pos = cache.levels[0].brith_point;
    c.spawn(PbrBundle {
        mesh: cache.get_character_mesh().clone(),
        material: cache.get_material("Covict", 0).clone(),
        transform: Transform::from_xyz(
            pos[0] as f32 * SCALE_RATIO * GRID_SIZE,
            (28 / 2) as f32 * SQRT_2 * SCALE_RATIO,
            -pos[1] as f32 * SCALE_RATIO * GRID_SIZE * SQRT_2,
        ),
        ..default()
    })
    .with_children(|p| {
        p.spawn(PbrBundle {
            mesh: cache.char_hand_mesh.clone(),
            material: cache.char_hand_material.clone(),
            transform: Transform::from_xyz(7.0 * SCALE_RATIO, -4.0 * SCALE_RATIO, 0.0),
            ..default()
        });
    })
    .insert(MaterialSprite::from("Covict", 0))
    .insert(SpriteAnimation::from_loop(&CHARACTER_FRAMES, 0.1))
    .insert(Character {
        direction: CharacterDirection::Down,
        action: CharacterAction::Idle,
        move_speed: 80.0,
    })
    .insert(NotShadowCaster::default())
    .insert(Name::new("Character"));
}

pub fn update_character_sprite(
    mut query: Query<(&mut MaterialSprite, &mut SpriteAnimation, &Character)>,
) {
    for (mut sprite, mut anima, char) in query.iter_mut() {
        match char.action {
            CharacterAction::Idle => match char.direction {
                CharacterDirection::Down => {
                    anima.update(TAG_IDLE_DOWN);
                }
                CharacterDirection::DownLeft => {
                    anima.update(TAG_IDLE_DOWN_RIGHT);
                    sprite.flip_x = true;
                }
                CharacterDirection::DownRight => {
                    anima.update(TAG_IDLE_DOWN_RIGHT);
                    sprite.flip_x = false;
                }
                CharacterDirection::Up => {
                    anima.update(TAG_IDLE_UP);
                }
                CharacterDirection::UpLeft => {
                    anima.update(TAG_IDLE_UP_RIGHT);
                    sprite.flip_x = true;
                }
                CharacterDirection::UpRight => {
                    anima.update(TAG_IDLE_UP_RIGHT);
                    sprite.flip_x = false;
                }
            },
            CharacterAction::Walking => match char.direction {
                CharacterDirection::Down => {
                    anima.update(TAG_WALKING_DOWN);
                }
                CharacterDirection::DownLeft => {
                    anima.update(TAG_WALKING_DOWN_RIGHT);
                    sprite.flip_x = true;
                }
                CharacterDirection::DownRight => {
                    anima.update(TAG_WALKING_DOWN_RIGHT);
                    sprite.flip_x = false;
                }
                CharacterDirection::Up => {
                    anima.update(TAG_WALKING_UP);
                }
                CharacterDirection::UpLeft => {
                    anima.update(TAG_WALKING_UP_RIGHT);
                    sprite.flip_x = true;
                }
                CharacterDirection::UpRight => {
                    anima.update(TAG_WALKING_UP_RIGHT);
                    sprite.flip_x = false;
                }
            },
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MoveDirection {
    x: MoveDirectionX,
    y: MoveDirectionY,
}

#[derive(Default, Debug, Copy, Clone)]
enum MoveDirectionX {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Default, Debug, Copy, Clone)]
enum MoveDirectionY {
    Up,
    Down,
    #[default]
    None,
}

impl MoveDirection {
    pub fn detect_key(&mut self, keyboard: &Input<KeyCode>) {
        if keyboard.pressed(KeyCode::W) {
            self.up()
        }
        if keyboard.pressed(KeyCode::S) {
            self.down()
        }
        if keyboard.pressed(KeyCode::A) {
            self.left()
        }
        if keyboard.pressed(KeyCode::D) {
            self.right()
        }
    }
    fn up(&mut self) {
        match self.y {
            MoveDirectionY::None => {
                self.y = MoveDirectionY::Up;
            }
            MoveDirectionY::Down => {
                self.y = MoveDirectionY::None;
            }
            _ => {}
        }
    }
    fn down(&mut self) {
        match self.y {
            MoveDirectionY::None => {
                self.y = MoveDirectionY::Down;
            }
            MoveDirectionY::Up => {
                self.y = MoveDirectionY::None;
            }
            _ => {}
        }
    }
    fn left(&mut self) {
        match self.x {
            MoveDirectionX::None => {
                self.x = MoveDirectionX::Left;
            }
            MoveDirectionX::Right => {
                self.x = MoveDirectionX::None;
            }
            _ => {}
        }
    }
    fn right(&mut self) {
        match self.x {
            MoveDirectionX::None => {
                self.x = MoveDirectionX::Right;
            }
            MoveDirectionX::Left => {
                self.x = MoveDirectionX::None;
            }
            _ => {}
        }
    }
}

pub fn character_move(
    mut query: Query<(&mut Transform, &mut Character)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    cache: Res<ResourceCache>,
) {
    for (mut t, mut char) in query.iter_mut() {
        let mut move_direction = MoveDirection::default();
        move_direction.detect_key(&keyboard);
        let old_pos = [
            t.translation.x / SCALE_RATIO,
            -(t.translation.z / SCALE_RATIO) / SQRT_2,
        ];
        let speed = time.delta_seconds() * char.move_speed;
        let to_grid_pos = |pos: [f32; 2]| -> [i32; 2] {
            // TODO 目前不知道为什么整体偏移了(8,-24)
            let pos = [pos[0] + 8.0, pos[1] + 24.0];
            [
                (pos[0] / GRID_SIZE).floor() as i32,
                (pos[1] / GRID_SIZE).floor() as i32,
            ]
        };
        let mut walking = true;
        match (move_direction.x, move_direction.y) {
            (MoveDirectionX::None, MoveDirectionY::Up) => {
                char.direction.to_up();
                let new_pos = [old_pos[0], old_pos[1] + speed];
                let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
                let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
                if cache.levels[0].contains_floor(need_detect_left_pos)
                    && cache.levels[0].contains_floor(need_detect_right_pos)
                {
                    t.translation.x = new_pos[0] * SCALE_RATIO;
                    t.translation.z = -new_pos[1] * SQRT_2 * SCALE_RATIO;
                }
            }
            (MoveDirectionX::None, MoveDirectionY::Down) => {
                char.direction.to_down();
                let new_pos = [old_pos[0], old_pos[1] - speed];
                let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
                let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
                if cache.levels[0].contains_floor(need_detect_left_pos)
                    && cache.levels[0].contains_floor(need_detect_right_pos)
                {
                    t.translation.x = new_pos[0] * SCALE_RATIO;
                    t.translation.z = -new_pos[1] * SQRT_2 * SCALE_RATIO;
                }
            }
            (MoveDirectionX::Left, MoveDirectionY::None) => {
                char.direction.to_left();
                let new_pos = [old_pos[0] - speed, old_pos[1]];
                let need_detect_top_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
                let need_detect_bottom_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
                if cache.levels[0].contains_floor(need_detect_top_pos)
                    && cache.levels[0].contains_floor(need_detect_bottom_pos)
                {
                    t.translation.x = new_pos[0] * SCALE_RATIO;
                    t.translation.z = -new_pos[1] * SQRT_2 * SCALE_RATIO;
                }
            }
            (MoveDirectionX::Right, MoveDirectionY::None) => {
                char.direction.to_right();
                let new_pos = [old_pos[0] + speed, old_pos[1]];
                let need_detect_top_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
                let need_detect_bottom_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
                if cache.levels[0].contains_floor(need_detect_top_pos)
                    && cache.levels[0].contains_floor(need_detect_bottom_pos)
                {
                    t.translation.x = new_pos[0] * SCALE_RATIO;
                    t.translation.z = -new_pos[1] * SQRT_2 * SCALE_RATIO;
                }
            }
            _ => {
                walking = false;
            }
        }

        // TODO 处理移动的方向和处理角色的朝向应该是分离的
        if walking {
            char.action = CharacterAction::Walking;
        } else {
            char.action = CharacterAction::Idle;
        }
    }
}

pub fn play_character_sound(
    query: Query<&MaterialSprite, With<Character>>,
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

pub fn camera_follow_character(
    character_query: Query<&Transform, (With<Character>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Character>)>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        for character_transform in character_query.iter() {
            camera_transform.translation.x = character_transform.translation.x;
            camera_transform.translation.y = character_transform.translation.y + 200.0;
            camera_transform.translation.z = character_transform.translation.z + 200.0;
        }
    }
}
