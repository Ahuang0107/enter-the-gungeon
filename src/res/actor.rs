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
    hp_full: u8,
    hp: u8,
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
            offset: [11.0, -1.0],
            hand_offset: [-4.0, -3.0],
            cursor_angle: 0.0,
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
        let y = self.pos[1] as f32 * SCALE_RATIO;
        let z = -y + ((28.0 / 2.0) * SCALE_RATIO);
        Vec3::new(x, y, z)
    }
    pub fn get_cur_hp(&self) -> u8 {
        self.hp
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
    pub fn get_cur_gun(&self) -> &Option<ResGun> {
        &self.gun
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
    pub cursor_angle: f32,
}

impl ResGun {
    pub fn get_gun_offset(&self) -> Vec3 {
        Vec3::new(
            self.offset[0] * SCALE_RATIO,
            self.offset[1] * SCALE_RATIO,
            0.0,
        )
    }
    pub fn get_gun_offset_flip(&self) -> Vec3 {
        Vec3::new(
            -self.offset[0] * SCALE_RATIO,
            self.offset[1] * SCALE_RATIO,
            0.0,
        )
    }
    pub fn get_hand_offset(&self) -> Vec3 {
        Vec3::new(
            self.hand_offset[0] * SCALE_RATIO,
            self.hand_offset[1] * SCALE_RATIO,
            0.0,
        ) + self.get_gun_offset()
    }
    pub fn get_hand_offset_flip(&self) -> Vec3 {
        Vec3::new(
            -self.hand_offset[0] * SCALE_RATIO,
            self.hand_offset[1] * SCALE_RATIO,
            0.0,
        ) + self.get_gun_offset_flip()
    }
}
