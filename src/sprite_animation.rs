use bevy::prelude::*;

/// 因为Animation的逻辑完全是封闭的，所以可以写成Plugin然后只暴露出一个Component
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sprite_animation);
    }
}

/// 切换动画的开始帧和结束帧的位置
#[derive(Component, Reflect)]
pub struct SpriteAnimation {
    timer: Timer,
    /// 不同 tag 对应的帧
    #[reflect(ignore)]
    tag_frames: std::collections::HashMap<&'static str, &'static [usize]>,
    /// 当前的 tag 和当前的帧的索引
    #[reflect(ignore)]
    current_frame: (&'static str, usize),
    is_loop: bool,
    finished: bool,
}

impl SpriteAnimation {
    pub fn from_loop(frames: &[(&'static str, &'static [usize])], interval: f32) -> Self {
        assert!(frames.len() > 0);
        let mut frames_map = std::collections::HashMap::new();
        for (name, frames) in frames {
            frames_map.insert(*name, *frames);
        }
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            tag_frames: frames_map,
            current_frame: (frames[0].0, 0),
            is_loop: true,
            finished: false,
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
    pub fn next_frame(&mut self) -> usize {
        let &frames = self.tag_frames.get(self.current_frame.0).unwrap();
        let len = frames.len();
        let mut current_frame_index = self.current_frame.1 + 1;
        if current_frame_index >= len {
            current_frame_index = 0;
            if !self.is_loop {
                self.finished = true;
            }
        }
        self.current_frame.1 = current_frame_index;
        self.tag_frames.get(self.current_frame.0).unwrap()[self.current_frame.1]
    }
    /// 判断当前是否在某一个状态
    pub fn if_tag(&self, tag: &str) -> bool {
        self.current_frame.0 == tag
    }
    pub fn if_finished(&self) -> bool {
        self.finished
    }
}

pub fn sprite_animation(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        if !animation.if_finished() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                sprite.index = animation.next_frame();
            }
        }
    }
}
