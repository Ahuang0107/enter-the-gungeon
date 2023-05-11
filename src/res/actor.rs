use std::f32::consts::SQRT_2;

use bevy::prelude::*;

use crate::res::SCALE_RATIO;

#[derive(Resource, Reflect, Default)]
pub struct ResActor {
    pos: [f32; 2],
    direction: ActorDirection,
    action: ActorAction,
    move_speed: f32,
    hp_full: u8,
    hp: u8,
    // only for debug inspect
    cursor_angle: f32,
    #[reflect(ignore)]
    gun: Option<ResGun>,
}

impl ResActor {
    /// 只是一个开发阶段直接使用制定配置的方法
    pub fn convict() -> Self {
        let default_hp = 6;
        Self {
            move_speed: 100.0,
            hp_full: default_hp,
            hp: default_hp,
            ..Default::default()
        }
    }
    /// 只是一个开发阶段直接使用制定配置的方法
    pub fn with_budget_revolver(mut self) -> Self {
        self.gun = Some(ResGun {
            name: String::from("Budget Revolver"),
            size: [16, 16],
            offset: [11.0, -2.0],
            hand_offset: [-4.0, -4.0],
        });
        self
    }
    pub fn update_pos(&mut self, pos: [f32; 2]) {
        self.pos = pos;
    }
    pub fn get_tilemap_pos(&self) -> [f32; 2] {
        self.pos
    }
    pub fn get_actual_pos(&self) -> Vec3 {
        Vec3::new(
            self.pos[0] as f32 * SCALE_RATIO,
            (28 / 2) as f32 * SQRT_2 * SCALE_RATIO,
            -self.pos[1] as f32 * SCALE_RATIO * SQRT_2,
        )
    }
    pub fn get_cur_hp(&self) -> u8 {
        self.hp
    }
    // only for debug inspect
    pub fn update_angle(&mut self, angle: f32) {
        self.cursor_angle = angle
    }
    pub fn get_move_speed(&self) -> f32 {
        self.move_speed
    }
    pub fn get_action(&self) -> ActorAction {
        self.action
    }
    pub fn get_direction(&self) -> ActorDirection {
        self.direction
    }
    pub fn get_cur_gun(&self) -> &Option<ResGun> {
        &self.gun
    }
    pub fn active_idle(&mut self) {
        self.action = ActorAction::Idle
    }
    pub fn active_walking(&mut self) {
        self.action = ActorAction::Walking
    }
    pub fn turn_up(&mut self) {
        self.direction = ActorDirection::Up
    }
    pub fn turn_down(&mut self) {
        self.direction = ActorDirection::Down
    }
    pub fn turn_left(&mut self) {
        match self.direction {
            ActorDirection::Up | ActorDirection::UpRight | ActorDirection::UpLeft => {
                self.direction = ActorDirection::UpLeft
            }
            ActorDirection::Down | ActorDirection::DownRight | ActorDirection::DownLeft => {
                self.direction = ActorDirection::DownLeft
            }
        }
    }
    pub fn turn_right(&mut self) {
        match self.direction {
            ActorDirection::Up | ActorDirection::UpRight | ActorDirection::UpLeft => {
                self.direction = ActorDirection::UpRight
            }
            ActorDirection::Down | ActorDirection::DownRight | ActorDirection::DownLeft => {
                self.direction = ActorDirection::DownRight
            }
        }
    }
}

#[derive(PartialEq, Reflect, Default, Copy, Clone)]
pub enum ActorAction {
    #[default]
    Idle,
    Walking,
}

#[derive(PartialEq, Reflect, Default, Copy, Clone)]
pub enum ActorDirection {
    #[default]
    Down,
    DownLeft,
    DownRight,
    Up,
    UpLeft,
    UpRight,
}

pub struct ResGun {
    pub name: String,
    pub size: [u32; 2],
    pub offset: [f32; 2],
    pub hand_offset: [f32; 2],
}
