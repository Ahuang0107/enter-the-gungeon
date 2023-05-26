use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct UiImageAnimation {
    timer: Timer,
    frames: Vec<Handle<Image>>,
    cur: usize,
    is_loop: bool,
    finished: bool,
    just_last: bool,
}

impl UiImageAnimation {
    pub fn from_loop(interval: f32, frames: Vec<Handle<Image>>) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            frames,
            cur: 0,
            is_loop: true,
            finished: false,
            just_last: false,
        }
    }
    fn if_finished(&self) -> bool {
        self.finished
    }
    pub fn if_just_last_frame(&self) -> bool {
        self.just_last
    }
}

pub fn update(time: Res<Time>, mut query: Query<(&mut UiImageAnimation, &mut UiImage)>) {
    for (mut anima, mut ui_image) in &mut query {
        if anima.just_last {
            anima.just_last = false;
        }
        if !anima.if_finished() {
            anima.timer.tick(time.delta());
            if anima.timer.just_finished() {
                let mut cur_frame_index = anima.cur + 1;
                if cur_frame_index >= anima.frames.len() {
                    cur_frame_index = 0;
                    if !anima.is_loop {
                        anima.finished = true;
                    }
                } else if cur_frame_index == anima.frames.len() - 1 {
                    anima.just_last = true;
                }
                if !anima.finished {
                    anima.cur = cur_frame_index;
                    ui_image.texture = anima.frames[anima.cur].clone();
                }
            }
        }
    }
}
