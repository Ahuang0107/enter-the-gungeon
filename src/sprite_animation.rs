use bevy::prelude::*;

use crate::resource::ResourceCache;

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct MaterialSprite {
    // 调整了逻辑，index对应的是cache中存储的material对应的index
    pub tag: String,
    pub index: u8,
    pub flip_x: bool,
    changed: bool,
}

impl MaterialSprite {
    pub fn from(tag: &str, index: u8) -> Self {
        Self {
            tag: tag.to_string(),
            index,
            flip_x: false,
            changed: true,
        }
    }
}

pub fn update_sprite(
    cache: Res<ResourceCache>,
    mut query: Query<(
        &mut Handle<StandardMaterial>,
        &mut Handle<Mesh>,
        &mut MaterialSprite,
    )>,
) {
    for (mut material_handle, mut mesh_handle, mut sprite) in query.iter_mut() {
        if sprite.changed {
            *material_handle = cache
                .get_material(sprite.tag.as_str(), sprite.index)
                .clone();
            if sprite.flip_x {
                *mesh_handle = cache.tile_28_deg_30_flip().clone();
            } else {
                *mesh_handle = cache.tile_28_deg_30().clone();
            }
            sprite.changed = false;
        }
    }
}

/// 切换动画的开始帧和结束帧的位置
#[derive(Component, Reflect)]
pub struct SpriteAnimation {
    timer: Timer,
    /// 不同 tag 对应的帧
    #[reflect(ignore)]
    tag_frames: std::collections::HashMap<&'static str, &'static [u8]>,
    /// 当前的 tag 和当前的帧的索引
    #[reflect(ignore)]
    current_frame: (&'static str, u8),
    is_loop: bool,
    finished: bool,
    just_last: bool,
}

#[allow(dead_code)]
impl SpriteAnimation {
    pub fn from_loop(frames_groups: &[(&'static str, &'static [u8])], interval: f32) -> Self {
        Self::from(frames_groups, interval, true)
    }
    pub fn from_once(frames: &'static [u8], interval: f32) -> Self {
        Self::from(&[("", frames)], interval, false)
    }
    fn from(frames: &[(&'static str, &'static [u8])], interval: f32, is_loop: bool) -> Self {
        assert!(frames.len() > 0);
        let mut frames_map = std::collections::HashMap::new();
        for (name, frames) in frames {
            frames_map.insert(*name, *frames);
        }
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            tag_frames: frames_map,
            current_frame: (frames[0].0, 0),
            is_loop,
            finished: false,
            just_last: false,
        }
    }
    /// 更新当前循环的 frame 的 tag，如果相比原来有变化则返回true，否则返回false
    pub fn update(&mut self, tag: &'static str) -> bool {
        if tag == self.current_frame.0 {
            false
        } else {
            self.current_frame = (tag, 0);
            true
        }
    }
    /// 更新到下一帧
    pub fn next_frame(&mut self) -> Option<u8> {
        let &frames = self.tag_frames.get(self.current_frame.0).unwrap();
        let len = frames.len() as u8;
        let mut current_frame_index = self.current_frame.1 + 1;
        if current_frame_index >= len {
            current_frame_index = 0;
            if !self.is_loop {
                self.finished = true;
            }
        } else if current_frame_index == len - 1 {
            self.just_last = true;
        }
        if self.finished {
            return None;
        } else {
            self.current_frame.1 = current_frame_index;
            Some(self.tag_frames.get(self.current_frame.0).unwrap()[self.current_frame.1 as usize])
        }
    }
    /// 判断当前是否在某一个状态
    pub fn if_tag(&self, tag: &str) -> bool {
        self.current_frame.0 == tag
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
    mut query: Query<(&mut SpriteAnimation, &mut MaterialSprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        if animation.just_last {
            animation.just_last = false;
        }
        if !animation.if_finished() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                if let Some(index) = animation.next_frame() {
                    sprite.index = index;
                    sprite.changed = true;
                }
            }
        }
    }
}
