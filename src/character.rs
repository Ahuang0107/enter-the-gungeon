use std::f32::consts::SQRT_2;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_kira_audio::AudioControl;
use rand::Rng;

use crate::res::{ActorAction, ActorDirection, Cache, ResActor, GRID_SIZE, SCALE_RATIO};
use crate::sprite_animation::{MaterialSprite, SpriteAnimation};

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
pub struct CopActor;

#[derive(Component)]
pub struct CopGun;

pub fn setup(mut c: Commands, cache: Res<Cache>, actor: ResMut<ResActor>) {
    c.spawn(PbrBundle {
        mesh: cache.get_character_mesh().clone(),
        material: cache.get_tile_material("Covict", 0).clone(),
        transform: Transform {
            translation: actor.get_actual_pos(),
            ..default()
        },
        ..default()
    })
    .with_children(|p| {
        if let Some(actor_gun) = actor.get_cur_gun() {
            let name = actor_gun.name.clone();
            let size = actor_gun.size;
            let offset = actor_gun.offset;
            let hand_offset = actor_gun.hand_offset;
            p.spawn(PbrBundle {
                mesh: cache.get_gun_mesh((size[0], size[1])).clone(),
                material: cache.get_gun_material(&name, 0).clone(),
                transform: Transform::from_xyz(
                    offset[0] * SCALE_RATIO,
                    offset[1] * SCALE_RATIO,
                    0.0,
                ),
                ..default()
            })
            .insert(CopGun)
            .insert(Name::new(name))
            .with_children(|p| {
                p.spawn(PbrBundle {
                    mesh: cache.char_hand_mesh.clone(),
                    material: cache.char_hand_material.clone(),
                    transform: Transform::from_xyz(
                        hand_offset[0] * SCALE_RATIO,
                        hand_offset[1] * SCALE_RATIO,
                        0.0,
                    ),
                    ..default()
                })
                .insert(Name::new("Hand"));
            });
        }
    })
    .insert(MaterialSprite::from("Covict", 0))
    .insert(SpriteAnimation::from_loop(&CHARACTER_FRAMES, 0.1))
    .insert(CopActor)
    .insert(NotShadowCaster::default())
    .insert(Name::new("Character"));
}

pub fn update_character_sprite(
    actor: Res<ResActor>,
    mut query: Query<(&mut MaterialSprite, &mut SpriteAnimation), With<CopActor>>,
) {
    for (mut sprite, mut anima) in query.iter_mut() {
        match actor.get_action() {
            ActorAction::Idle => match actor.get_direction() {
                ActorDirection::Down => {
                    anima.update(TAG_IDLE_DOWN);
                }
                ActorDirection::DownLeft => {
                    anima.update(TAG_IDLE_DOWN_RIGHT);
                    sprite.flip_x = true;
                }
                ActorDirection::DownRight => {
                    anima.update(TAG_IDLE_DOWN_RIGHT);
                    sprite.flip_x = false;
                }
                ActorDirection::Up => {
                    anima.update(TAG_IDLE_UP);
                }
                ActorDirection::UpLeft => {
                    anima.update(TAG_IDLE_UP_RIGHT);
                    sprite.flip_x = true;
                }
                ActorDirection::UpRight => {
                    anima.update(TAG_IDLE_UP_RIGHT);
                    sprite.flip_x = false;
                }
            },
            ActorAction::Walking => match actor.get_direction() {
                ActorDirection::Down => {
                    anima.update(TAG_WALKING_DOWN);
                }
                ActorDirection::DownLeft => {
                    anima.update(TAG_WALKING_DOWN_RIGHT);
                    sprite.flip_x = true;
                }
                ActorDirection::DownRight => {
                    anima.update(TAG_WALKING_DOWN_RIGHT);
                    sprite.flip_x = false;
                }
                ActorDirection::Up => {
                    anima.update(TAG_WALKING_UP);
                }
                ActorDirection::UpLeft => {
                    anima.update(TAG_WALKING_UP_RIGHT);
                    sprite.flip_x = true;
                }
                ActorDirection::UpRight => {
                    anima.update(TAG_WALKING_UP_RIGHT);
                    sprite.flip_x = false;
                }
            },
        }
    }
}

pub fn character_move(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut actor: ResMut<ResActor>,
    cache: Res<Cache>,
) {
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

    let mut move_direction = MoveDirection::default();
    move_direction.detect_key(&keyboard);
    let old_pos = [
        actor.get_actual_pos().x / SCALE_RATIO,
        -(actor.get_actual_pos().z / SCALE_RATIO) / SQRT_2,
    ];
    let speed = time.delta_seconds() * actor.get_move_speed();
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
            actor.turn_up();
            let new_pos = [old_pos[0], old_pos[1] + speed];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.update_pos(new_pos);
            }
        }
        (MoveDirectionX::None, MoveDirectionY::Down) => {
            actor.turn_down();
            let new_pos = [old_pos[0], old_pos[1] - speed];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.update_pos(new_pos);
            }
        }
        (MoveDirectionX::Left, MoveDirectionY::None) => {
            actor.turn_left();
            let new_pos = [old_pos[0] - speed, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.update_pos(new_pos);
            }
        }
        (MoveDirectionX::Right, MoveDirectionY::None) => {
            actor.turn_right();
            let new_pos = [old_pos[0] + speed, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.update_pos(new_pos);
            }
        }
        _ => {
            walking = false;
        }
    }

    // TODO 处理移动的方向和处理角色的朝向应该是分离的
    if walking {
        actor.active_walking()
    } else {
        actor.active_idle()
    }
}

pub fn play_character_sound(
    query: Query<&MaterialSprite, With<CopActor>>,
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
