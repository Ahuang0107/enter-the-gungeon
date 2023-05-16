use std::collections::HashMap;

use bevy::prelude::*;

use crate::res::Cache;

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct ActorMaterialSprite {
    pub name: String,
    // 调整了逻辑，index对应的是cache中存储的material对应的index
    pub tag: String,
    pub index: usize,
    pub flip_x: bool,
    changed: bool,
}

impl ActorMaterialSprite {
    pub fn from(name: &str, tag: &str, index: usize) -> Self {
        Self {
            name: name.to_string(),
            tag: tag.to_string(),
            index,
            flip_x: false,
            changed: true,
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
            *material_handle = cache
                .get_actor_material(&sprite.name, &sprite.tag, sprite.index)
                .clone();
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
    /// 不同 tag 对应的帧长度
    #[reflect(ignore)]
    tag_frames: HashMap<String, usize>,
    cur_tag: String,
    cur_index: usize,
    is_loop: bool,
    finished: bool,
    just_last: bool,
}

#[allow(dead_code)]
impl ActorSpriteAnimation {
    pub fn from_loop(frames_groups: HashMap<String, usize>, interval: f32) -> Self {
        Self::from(frames_groups, interval, true)
    }
    pub fn from_once(frames_size: usize, interval: f32) -> Self {
        Self::from(
            HashMap::from([(String::new(), frames_size)]),
            interval,
            false,
        )
    }
    fn from(frames: HashMap<String, usize>, interval: f32, is_loop: bool) -> Self {
        assert!(frames.len() > 0);
        let mut cur_tag = None;
        for (name, _) in frames.iter() {
            if cur_tag.is_none() {
                cur_tag = Some(name.clone());
                break;
            }
        }
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            tag_frames: frames,
            cur_tag: cur_tag.unwrap(),
            cur_index: 0,
            is_loop,
            finished: false,
            just_last: false,
        }
    }
    /// 更新当前循环的 frame 的 tag，如果相比原来有变化则返回true，否则返回false
    pub fn update(&mut self, tag: &str) -> bool {
        if tag == self.cur_tag {
            false
        } else {
            self.cur_tag = tag.to_string();
            self.cur_index = 0;
            true
        }
    }
    /// 更新到下一帧
    pub fn next_frame(&mut self) -> Option<(&str, usize)> {
        let &frames_size = self.tag_frames.get(&self.cur_tag).unwrap();
        let mut cur_frame_index = self.cur_index + 1;
        if cur_frame_index >= frames_size {
            cur_frame_index = 0;
            if !self.is_loop {
                self.finished = true;
            }
        } else if cur_frame_index == frames_size - 1 {
            self.just_last = true;
        }
        if self.finished {
            return None;
        } else {
            self.cur_index = cur_frame_index;
            Some((self.cur_tag.as_str(), self.cur_index))
        }
    }
    /// 判断当前是否在某一个状态
    pub fn if_tag(&self, tag: &str) -> bool {
        self.cur_tag == tag
    }
    pub fn if_finished(&self) -> bool {
        self.finished
    }
    pub fn if_just_last_frame(&self) -> bool {
        self.just_last
    }
}

pub fn sprite_animation(
    time: Res<Time>,
    mut query: Query<(&mut ActorSpriteAnimation, &mut ActorMaterialSprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        if animation.just_last {
            animation.just_last = false;
        }
        if !animation.if_finished() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                if let Some((tag, index)) = animation.next_frame() {
                    sprite.tag = tag.to_string();
                    sprite.index = index;
                    sprite.changed = true;
                }
            }
        }
    }
}
