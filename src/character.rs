use std::f32::consts::{PI, SQRT_2};

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_kira_audio::AudioControl;
use rand::Rng;

use crate::cursor::ResCursor;
use crate::res::{
    ActorAction, ActorDirection, ActorGunHand, Cache, ResActor, GRID_SIZE, SCALE_RATIO,
};
use crate::sprite_animation::{ActorMaterialSprite, ActorSpriteAnimation};

pub enum ActorTag {
    IdleF1h,
    IdleB1h,
    IdleFs1h,
    IdleBs0h,
    WalkF1h,
    WalkB1h,
    WalkFs1h,
    WalkBs0h,
}

impl ActorTag {
    pub fn tag(&self) -> &str {
        match self {
            Self::IdleF1h => "idle-f-1h",
            Self::IdleB1h => "idle-b-1h",
            Self::IdleFs1h => "idle-fs-1h",
            Self::IdleBs0h => "idle-bs-0h",
            Self::WalkF1h => "walk-f-1h",
            Self::WalkB1h => "walk-b-1h",
            Self::WalkFs1h => "walk-fs-1h",
            Self::WalkBs0h => "walk-bs-0h",
        }
    }
}

#[derive(Component)]
pub struct CopActor;

#[derive(Component)]
pub struct CopGun {
    mesh: Handle<Mesh>,
    mesh_flip: Handle<Mesh>,
}

#[derive(Component)]
pub struct CopHand;

pub fn setup(mut c: Commands, cache: Res<Cache>, actor: ResMut<ResActor>) {
    c.spawn(PbrBundle {
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
                mesh: cache.get_gun_mesh((size[0], size[1]), false).clone(),
                material: cache.get_gun_material(&name, 0).clone(),
                transform: Transform::from_xyz(
                    offset[0] * SCALE_RATIO,
                    offset[1] * SCALE_RATIO,
                    0.0,
                ),
                ..default()
            })
            .insert(CopGun {
                mesh: cache.get_gun_mesh((size[0], size[1]), false).clone(),
                mesh_flip: cache.get_gun_mesh((size[0], size[1]), true).clone(),
            })
            .insert(NotShadowCaster::default())
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
                .insert(CopHand)
                .insert(NotShadowCaster::default())
                .insert(Name::new("Hand"));
            });
        }
    })
    .insert(ActorMaterialSprite::default())
    .insert(ActorSpriteAnimation::from_loop("Convict", "idle-f-2h", 0.1))
    .insert(CopActor)
    .insert(NotShadowCaster::default())
    .insert(Name::new("Character"));
}

pub fn update_gun_direction(
    actor: Res<ResActor>,
    mut gun_query: Query<(&mut Transform, &mut Handle<Mesh>, &CopGun), Without<CopHand>>,
    mut hand_query: Query<(&mut Transform, &CopHand), Without<CopGun>>,
) {
    for (mut t, _) in hand_query.iter_mut() {
        match actor.get_gun_hand() {
            ActorGunHand::Left => {
                t.translation.x = t.translation.x.abs();
            }
            ActorGunHand::Right => {
                t.translation.x = -t.translation.x.abs();
            }
        }
    }
    for (mut t, mut mesh, cop_gun) in gun_query.iter_mut() {
        match actor.get_gun_hand() {
            ActorGunHand::Left => {
                t.translation.x = -t.translation.x.abs();
                *mesh = cop_gun.mesh_flip.clone();
            }
            ActorGunHand::Right => {
                t.translation.x = t.translation.x.abs();
                *mesh = cop_gun.mesh.clone();
            }
        }
    }
}

pub fn update_character_sprite(
    actor: Res<ResActor>,
    mut query: Query<(&mut ActorMaterialSprite, &mut ActorSpriteAnimation), With<CopActor>>,
) {
    for (mut sprite, mut anima) in query.iter_mut() {
        match actor.get_action() {
            ActorAction::Idle => match actor.get_direction() {
                ActorDirection::Down => {
                    anima.update(ActorTag::IdleF1h.tag());
                    match actor.get_gun_hand() {
                        ActorGunHand::Left => sprite.flip_x = true,
                        ActorGunHand::Right => sprite.flip_x = false,
                    }
                }
                ActorDirection::DownLeft => {
                    anima.update(ActorTag::IdleFs1h.tag());
                    sprite.flip_x = true;
                }
                ActorDirection::DownRight => {
                    anima.update(ActorTag::IdleFs1h.tag());
                    sprite.flip_x = false;
                }
                ActorDirection::Up => {
                    anima.update(ActorTag::IdleB1h.tag());
                    match actor.get_gun_hand() {
                        ActorGunHand::Left => sprite.flip_x = true,
                        ActorGunHand::Right => sprite.flip_x = false,
                    }
                }
                ActorDirection::UpLeft => {
                    anima.update(ActorTag::IdleBs0h.tag());
                    sprite.flip_x = true;
                }
                ActorDirection::UpRight => {
                    anima.update(ActorTag::IdleBs0h.tag());
                    sprite.flip_x = false;
                }
            },
            ActorAction::Walking => match actor.get_direction() {
                ActorDirection::Down => {
                    anima.update(ActorTag::WalkF1h.tag());
                    match actor.get_gun_hand() {
                        ActorGunHand::Left => sprite.flip_x = true,
                        ActorGunHand::Right => sprite.flip_x = false,
                    }
                }
                ActorDirection::DownLeft => {
                    anima.update(ActorTag::WalkFs1h.tag());
                    sprite.flip_x = true;
                }
                ActorDirection::DownRight => {
                    anima.update(ActorTag::WalkFs1h.tag());
                    sprite.flip_x = false;
                }
                ActorDirection::Up => {
                    anima.update(ActorTag::WalkB1h.tag());
                    match actor.get_gun_hand() {
                        ActorGunHand::Left => sprite.flip_x = true,
                        ActorGunHand::Right => sprite.flip_x = false,
                    }
                }
                ActorDirection::UpLeft => {
                    anima.update(ActorTag::WalkBs0h.tag());
                    sprite.flip_x = true;
                }
                ActorDirection::UpRight => {
                    anima.update(ActorTag::WalkBs0h.tag());
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
    cursor: Res<ResCursor>,
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
    let mut old_pos = [
        actor.get_actual_pos().x / SCALE_RATIO,
        actor.get_actual_pos().y / SCALE_RATIO,
    ];
    let speed = time.delta_seconds() * actor.get_move_speed();
    let to_grid_pos = |pos: [f32; 2]| -> [i32; 2] {
        // TODO 目前不知道为什么整体偏移了(8,-24)
        let pos = [pos[0] + 8.0, pos[1] + 8.0];
        [
            (pos[0] / GRID_SIZE).floor() as i32,
            (pos[1] / GRID_SIZE).floor() as i32,
        ]
    };
    let mut walking = true;
    // TODO 这里暂时没有抽象好，回头有空可以再回来改
    match (move_direction.x, move_direction.y) {
        (MoveDirectionX::None, MoveDirectionY::Up) => {
            let new_pos = [old_pos[0], old_pos[1] + speed];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::None, MoveDirectionY::Down) => {
            let new_pos = [old_pos[0], old_pos[1] - speed];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::Left, MoveDirectionY::None) => {
            let new_pos = [old_pos[0] - speed, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::Right, MoveDirectionY::None) => {
            let new_pos = [old_pos[0] + speed, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::Left, MoveDirectionY::Up) => {
            let ratio = SQRT_2 / 2.0;
            let new_pos = [old_pos[0], old_pos[1] + speed * ratio];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.set_tilemap_pos(new_pos);
                old_pos = new_pos;
            }
            let new_pos = [old_pos[0] - speed * ratio, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::Left, MoveDirectionY::Down) => {
            let ratio = SQRT_2 / 2.0;
            let new_pos = [old_pos[0], old_pos[1] - speed * ratio];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.set_tilemap_pos(new_pos);
                old_pos = new_pos;
            }
            let new_pos = [old_pos[0] - speed * ratio, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::Right, MoveDirectionY::Up) => {
            let ratio = SQRT_2 / 2.0;
            let new_pos = [old_pos[0], old_pos[1] + speed * ratio];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.set_tilemap_pos(new_pos);
                old_pos = new_pos;
            }
            let new_pos = [old_pos[0] + speed * ratio, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        (MoveDirectionX::Right, MoveDirectionY::Down) => {
            let ratio = SQRT_2 / 2.0;
            let new_pos = [old_pos[0], old_pos[1] - speed * ratio];
            let need_detect_left_pos = to_grid_pos([new_pos[0] - 7.0, new_pos[1]]);
            let need_detect_right_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_left_pos)
                && cache.levels[0].contains_floor(need_detect_right_pos)
            {
                actor.set_tilemap_pos(new_pos);
                old_pos = new_pos;
            }
            let new_pos = [old_pos[0] + speed * ratio, old_pos[1]];
            let need_detect_top_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            let need_detect_bottom_pos = to_grid_pos([new_pos[0] + 7.0, new_pos[1]]);
            if cache.levels[0].contains_floor(need_detect_top_pos)
                && cache.levels[0].contains_floor(need_detect_bottom_pos)
            {
                actor.set_tilemap_pos(new_pos);
            }
        }
        _ => {
            walking = false;
        }
    }

    // 判断actor当前状态
    if walking {
        actor.active_walking()
    } else {
        actor.active_idle()
    }

    // 判断actor当前朝向
    {
        let cursor_pos = cursor.get_tilemap_pos();
        let actor_pos = actor.get_tilemap_pos();
        let radians = Vec2::X.angle_between(
            Vec2::new(cursor_pos[0], cursor_pos[1]) - Vec2::new(actor_pos[0], actor_pos[1]),
        );
        let angle = (radians * 180.0 / PI).round();
        actor.update_cursor_angle(angle);
    }
}

pub fn play_character_sound(
    query: Query<&ActorMaterialSprite, With<CopActor>>,
    asset_server: Res<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
) {
    if !audio.is_playing_sound() {
        for sprite in query.iter() {
            if let Some(index) = sprite.index() {
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
