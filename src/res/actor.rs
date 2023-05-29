use std::f32::consts::{PI, SQRT_2};

use bevy::prelude::*;

use crate::res::SCALE_RATIO;

#[derive(Resource, Reflect, Default)]
pub struct ResActor {
    /// position in a virtual 2d world which used to calculate collision
    pos: [f32; 2],
    action: ActorAction,
    cursor_angle: f32,
    direction: ActorDirection,
    gun_hand: ActorGunHand,
    move_speed: f32,
    status: ActorStatus,
    #[reflect(ignore)]
    gun: Option<ResGun>,
}

impl ResActor {
    /// 只是一个开发阶段直接使用制定配置的方法
    pub fn convict() -> Self {
        let default_hp = 6;
        Self {
            move_speed: 100.0,
            status: ActorStatus {
                hp_full: default_hp,
                hp: default_hp,
                blanks: 2,
                keys: 10,
                money: 10,
            },
            ..Default::default()
        }
    }
    /// 只是一个开发阶段直接使用制定配置的方法
    pub fn with_budget_revolver(mut self) -> Self {
        self.gun = Some(ResGun {
            name: String::from("Budget Revolver"),
            size: [16, 16],
            offset: [11.0, -1.0],
            hand_offset: [-4.0, -3.0],
            cursor_angle: 0.0,
            fire_offset: [7.0, 2.0],
        });
        self
    }
    pub fn set_tilemap_pos(&mut self, pos: [f32; 2]) {
        self.pos = pos;
    }
    pub fn get_tilemap_pos(&self) -> [f32; 2] {
        self.pos
    }
    pub fn get_actual_pos(&self) -> Vec3 {
        let x = self.pos[0] as f32 * SCALE_RATIO;
        let offset = (28.0 / 2.0) * SCALE_RATIO;
        // actor实际上需要保持在floor之上，所以整体向镜头靠近
        let y = 0.0 + offset;
        let z = -self.pos[1] as f32 * SCALE_RATIO * SQRT_2 + offset;
        Vec3::new(x, y, z)
    }
    pub fn get_status(&self) -> &ActorStatus {
        &self.status
    }
    /// gun需要旋转的角度
    fn get_gun_radians(&self) -> f32 {
        match self.get_gun_hand() {
            ActorGunHand::Left => PI * (self.cursor_angle - 180.0) / 180.0,
            ActorGunHand::Right => PI * self.cursor_angle / 180.0,
        }
    }
    pub fn update_cursor_angle(&mut self, angle: f32) {
        self.cursor_angle = angle;

        enum CursorDirection {
            TopSlightlyLeft,
            TopSlightlyRight,
            TopLeft,
            TopRight,
            Left,
            Right,
            BottomSlightlyLeft,
            BottomSlightlyRight,
        }

        impl CursorDirection {
            fn from_angle(angle: f32) -> Self {
                if angle >= -60.0 && angle < 30.0 {
                    Self::Right
                } else if angle >= 30.0 && angle < 60.0 {
                    Self::TopRight
                } else if angle >= 60.0 && angle < 90.0 {
                    Self::TopSlightlyRight
                } else if angle >= 90.0 && angle < 120.0 {
                    Self::TopSlightlyLeft
                } else if angle >= 120.0 && angle < 150.0 {
                    Self::TopLeft
                } else if angle >= -90.0 && angle < -60.0 {
                    Self::BottomSlightlyRight
                } else if angle >= -120.0 && angle < -90.0 {
                    Self::BottomSlightlyLeft
                } else {
                    Self::Left
                }
            }
        }

        match CursorDirection::from_angle(angle) {
            CursorDirection::BottomSlightlyLeft | CursorDirection::BottomSlightlyRight => {
                self.turn_down();
            }
            CursorDirection::Right => {
                self.turn_down();
                self.turn_right();
            }
            CursorDirection::TopRight => {
                self.turn_up();
                self.turn_right();
            }
            CursorDirection::TopSlightlyLeft | CursorDirection::TopSlightlyRight => {
                self.turn_up();
            }
            CursorDirection::TopLeft => {
                self.turn_up();
                self.turn_left();
            }
            CursorDirection::Left => {
                self.turn_down();
                self.turn_left();
            }
        }
        if let Some(ref mut gun) = &mut self.gun {
            gun.cursor_angle = angle;
        }
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
    pub fn get_gun_hand(&self) -> ActorGunHand {
        self.gun_hand
    }
    pub fn get_cur_gun(&self) -> Option<&ResGun> {
        if let Some(gun) = &self.gun {
            Some(gun)
        } else {
            None
        }
    }
    pub fn active_idle(&mut self) {
        self.action = ActorAction::Idle
    }
    pub fn active_walking(&mut self) {
        self.action = ActorAction::Walking
    }
    fn turn_up(&mut self) {
        self.direction = ActorDirection::Up
    }
    fn turn_down(&mut self) {
        self.direction = ActorDirection::Down
    }
    fn turn_left(&mut self) {
        match self.direction {
            ActorDirection::Up | ActorDirection::UpRight | ActorDirection::UpLeft => {
                self.direction = ActorDirection::UpLeft
            }
            ActorDirection::Down | ActorDirection::DownRight | ActorDirection::DownLeft => {
                self.direction = ActorDirection::DownLeft
            }
        }
        self.gun_hand = ActorGunHand::Left;
    }
    fn turn_right(&mut self) {
        match self.direction {
            ActorDirection::Up | ActorDirection::UpRight | ActorDirection::UpLeft => {
                self.direction = ActorDirection::UpRight
            }
            ActorDirection::Down | ActorDirection::DownRight | ActorDirection::DownLeft => {
                self.direction = ActorDirection::DownRight
            }
        }
        self.gun_hand = ActorGunHand::Right;
    }
    pub fn get_fire_offset(&self) -> Option<Vec3> {
        if let Some(gun) = &self.gun {
            // 这里得到的所有offset都是相对于actor的
            let (hand_offset, fire_offset) = match self.get_gun_hand() {
                ActorGunHand::Left => (gun.get_hand_offset(true), gun.get_fire_offset(true)),
                ActorGunHand::Right => (gun.get_hand_offset(false), gun.get_fire_offset(false)),
            };
            let gun_radians = self.get_gun_radians();
            let relative_offset = (fire_offset - hand_offset)
                .truncate()
                .rotate(Vec2::from_angle(gun_radians));
            Some(relative_offset.extend(0.0) + hand_offset)
        } else {
            None
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

#[derive(PartialEq, Reflect, Default, Copy, Clone)]
pub enum ActorGunHand {
    #[default]
    Left,
    Right,
}

pub struct ResGun {
    pub name: String,
    pub size: [u32; 2],
    /// 枪械相对角色的位置偏移
    offset: [f32; 2],
    /// 手的位置相对枪械的偏移，也就是手握在枪械哪里的信息
    hand_offset: [f32; 2],
    cursor_angle: f32,
    /// 发射bullet的位置相对枪械的偏移
    fire_offset: [f32; 2],
}

impl ResGun {
    /// gun相对actor的位移
    /// 用来直接设置gun的位置
    pub fn get_gun_offset(&self, flip: bool) -> Vec3 {
        Vec3::new(
            if flip { -1.0 } else { 1.0 } * self.offset[0] * SCALE_RATIO,
            self.offset[1] * SCALE_RATIO,
            0.0,
        )
    }
    /// hand相对actor的位移
    /// 用来直接设置hand的位置
    pub fn get_hand_offset(&self, flip: bool) -> Vec3 {
        Vec3::new(
            if flip { -1.0 } else { 1.0 } * self.hand_offset[0] * SCALE_RATIO,
            self.hand_offset[1] * SCALE_RATIO,
            0.0,
        ) + self.get_gun_offset(flip)
    }
    /// fire相对actor的位移
    pub fn get_fire_offset(&self, flip: bool) -> Vec3 {
        Vec3::new(
            if flip { -1.0 } else { 1.0 } * self.fire_offset[0] * SCALE_RATIO,
            self.fire_offset[1] * SCALE_RATIO,
            0.0,
        ) + self.get_gun_offset(flip)
    }
    /// gun跟随cursor需要的旋转
    /// 用来对gun做瞄准方向的旋转
    pub fn get_rotation(&self, flip: bool) -> Quat {
        if self.cursor_angle == 0.0 || self.cursor_angle == 180.0 {
            Quat::default()
        } else {
            if flip {
                Quat::from_rotation_z(PI / (180.0 / (180.0 + self.cursor_angle)))
            } else {
                Quat::from_rotation_z(PI / (180.0 / self.cursor_angle))
            }
        }
    }
}

#[derive(Reflect, Default)]
pub struct ActorStatus {
    hp_full: u8,
    hp: u8,
    blanks: u8,
    keys: u8,
    money: u16,
}

impl ActorStatus {
    pub fn get_cur_hp(&self) -> u8 {
        self.hp
    }
    pub fn get_blanks(&self) -> u8 {
        self.blanks
    }
    pub fn get_keys(&self) -> u8 {
        self.keys
    }
    pub fn get_money(&self) -> u16 {
        self.money
    }
}
