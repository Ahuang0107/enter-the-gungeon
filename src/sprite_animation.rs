use bevy::prelude::*;

use crate::res::Cache;

#[derive(Clone, Debug, Default, Reflect)]
pub struct ActorSpriteDetail {
    name: String,
    tag: String,
    index: usize,
}

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct ActorMaterialSprite {
    /// only for display
    /// always update from ActorSpriteAnimation
    #[reflect(ignore)]
    detail: Option<ActorSpriteDetail>,
    material: Option<Handle<StandardMaterial>>,
    pub flip_x: bool,
    changed: bool,
}

impl ActorMaterialSprite {
    pub fn update(&mut self, material: &Handle<StandardMaterial>, detail: &ActorSpriteDetail) {
        self.material = Some(material.clone());
        self.changed = true;
        self.detail = Some(detail.clone());
    }
    pub fn index(&self) -> Option<usize> {
        if let Some(detail) = &self.detail {
            Some(detail.index)
        } else {
            None
        }
    }
}

pub fn update_sprite(
    cache: Res<Cache>,
    mut query: Query<(
        &mut Handle<StandardMaterial>,
        &mut Handle<Mesh>,
        &mut ActorMaterialSprite,
    )>,
) {
    for (mut material_handle, mut mesh_handle, mut sprite) in query.iter_mut() {
        if sprite.changed {
            if let Some(material) = &sprite.material {
                *material_handle = material.clone();
            }
            if sprite.flip_x {
                *mesh_handle = cache.get_character_mesh_flip().clone();
            } else {
                *mesh_handle = cache.get_character_mesh().clone();
            }
            sprite.changed = false;
        }
    }
}

/// 切换动画的开始帧和结束帧的位置
#[derive(Component, Reflect)]
pub struct ActorSpriteAnimation {
    timer: Timer,
    cur: ActorSpriteDetail,
    is_loop: bool,
    finished: bool,
    just_last: bool,
}

#[allow(dead_code)]
impl ActorSpriteAnimation {
    pub fn from_loop(name: &str, tag: &str, interval: f32) -> Self {
        Self::from(name, tag, interval, true)
    }
    pub fn from_once(name: &str, tag: &str, interval: f32) -> Self {
        Self::from(name, tag, interval, false)
    }
    fn from(name: &str, tag: &str, interval: f32, is_loop: bool) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            cur: ActorSpriteDetail {
                name: name.to_string(),
                tag: tag.to_string(),
                index: 0,
            },
            is_loop,
            finished: false,
            just_last: false,
        }
    }
    /// 更新当前循环的 frame 的 tag
    pub fn update(&mut self, tag: &str) {
        if !self.if_tag(tag) {
            self.cur.tag = tag.to_string();
            self.cur.index = 0;
        }
    }
    /// 判断当前是否在某一个状态
    fn if_tag(&self, tag: &str) -> bool {
        self.cur.tag == tag
    }
    fn if_finished(&self) -> bool {
        self.finished
    }
    pub fn if_just_last_frame(&self) -> bool {
        self.just_last
    }
}

pub fn sprite_animation(
    time: Res<Time>,
    cache: Res<Cache>,
    mut query: Query<(&mut ActorSpriteAnimation, &mut ActorMaterialSprite)>,
) {
    for (mut anima, mut sprite) in &mut query {
        if anima.just_last {
            anima.just_last = false;
        }
        if !anima.if_finished() {
            anima.timer.tick(time.delta());
            if anima.timer.just_finished() {
                let frames = cache.get_actor_materials(&anima.cur.name, &anima.cur.tag);
                let mut cur_frame_index = anima.cur.index + 1;
                if cur_frame_index >= frames.len() {
                    cur_frame_index = 0;
                    if !anima.is_loop {
                        anima.finished = true;
                    }
                } else if cur_frame_index == frames.len() - 1 {
                    anima.just_last = true;
                }
                if !anima.finished {
                    anima.cur.index = cur_frame_index;
                    if let Some(frame) = frames.get(anima.cur.index) {
                        sprite.update(frame, &anima.cur);
                    }
                }
            }
        }
    }
}
