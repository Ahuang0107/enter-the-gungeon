use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct OnceSpriteAnimation {
    timer: Timer,
    frames: Vec<Handle<StandardMaterial>>,
    cur: usize,
    finished: bool,
}

impl OnceSpriteAnimation {
    pub fn new(interval: f32, frames: Vec<Handle<StandardMaterial>>) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            frames,
            cur: 0,
            finished: false,
        }
    }
    fn if_finished(&self) -> bool {
        self.finished
    }
}

pub fn update(
    time: Res<Time>,
    mut query: Query<(&mut OnceSpriteAnimation, &mut Handle<StandardMaterial>)>,
) {
    for (mut anima, mut material) in &mut query {
        if !anima.if_finished() {
            anima.timer.tick(time.delta());
            if anima.timer.just_finished() {
                let mut cur_frame_index = anima.cur + 1;
                if cur_frame_index >= anima.frames.len() {
                    cur_frame_index = 0;
                    anima.finished = true;
                }
                if !anima.finished {
                    anima.cur = cur_frame_index;
                    *material = anima.frames[anima.cur].clone();
                }
            }
        }
    }
}

pub fn despawn_finished(mut c: Commands, query: Query<(&OnceSpriteAnimation, Entity)>) {
    for (anima, e) in query.iter() {
        if anima.if_finished() {
            c.entity(e).despawn_recursive();
        }
    }
}
