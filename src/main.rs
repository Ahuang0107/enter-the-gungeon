use crate::resource::ResourceCache;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::prelude::*;
use bevy_task_queue::TaskQueue;

mod character;
mod debug;
mod resource;
mod sprite_animation;
mod tilemap;
mod utils;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

impl AppState {
    pub fn next_state(&self) -> Option<Self> {
        match self {
            AppState::Loading => Some(AppState::InGame),
            _ => None,
        }
    }
}

pub fn auto_next_state(
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    task_queue: Res<TaskQueue>,
) {
    if let Some(next) = app_state.0.next_state() {
        if task_queue.is_empty() {
            debug!("task queue is empty, auto change to {next:?}");
            next_state.set(next);
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugin(AudioPlugin)
    .add_plugin(debug::DebugPlugin);
    app.insert_resource(ClearColor(Color::rgba_u8(3, 12, 14, 255)));
    app.add_state::<AppState>();
    app.insert_resource(ResourceCache::default());
    app.insert_resource(TaskQueue::new());
    app.add_startup_system(setup_camera);
    app.add_system(auto_next_state);
    // app.add_system(sprite_animation::update_sprite);
    // app.add_system(sprite_animation::sprite_animation);

    app.add_system((resource::initial_texture_atlases).in_schedule(OnEnter(AppState::Loading)));
    app.add_system((tilemap::setup).in_schedule(OnEnter(AppState::InGame)));
    // app.add_system((character::setup).in_schedule(OnEnter(AppState::InGame)));
    // app.add_systems(
    //     (
    //         character::update_character_sprite,
    //         character::play_character_sound,
    //         character::character_move,
    //     )
    //         .in_set(OnUpdate(AppState::InGame)),
    // );

    app.run();
}

fn setup_camera(mut c: Commands) {
    c.spawn(Camera3dBundle {
        projection: OrthographicProjection { ..default() }.into(),
        transform: Transform::from_xyz(0.0, 0.0, 20.0).with_scale(Vec3::splat(0.05)),
        ..default()
    });
}
