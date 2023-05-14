use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::prelude::*;

use bevy_task_queue::TaskQueue;
use res::{Cache, ResActor};

mod character;
mod cursor;
mod debug;
mod res;
mod sprite_animation;
mod tilemap;
mod ui;
mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum AppState {
    #[default]
    Title,
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
            .set(LogPlugin {
                filter: String::from("wgpu=error,enter_the_gungeon=debug,symphonia_core=warn"),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugin(AudioPlugin)
    .add_plugin(debug::DebugPlugin)
    .add_plugin(cursor::CursorDetectPlugin);
    app.insert_resource(ClearColor(Color::rgba_u8(3, 12, 14, 255)));
    app.add_state::<AppState>();
    app.insert_resource(Cache::default());
    app.insert_resource(ResActor::convict().with_budget_revolver());
    app.insert_resource(TaskQueue::new());
    app.add_startup_system(setup_camera);
    app.add_system(auto_next_state);
    app.add_system(sprite_animation::update_sprite);
    app.add_system(sprite_animation::sprite_animation);

    app.add_system((ui::title::setup).in_schedule(OnEnter(AppState::Title)));
    app.add_system((ui::title::detect_start).in_set(OnUpdate(AppState::Title)));
    app.add_system((ui::title::dismount).in_schedule(OnExit(AppState::Title)));

    app.add_system((res::reset_res).in_schedule(OnEnter(AppState::Loading)));

    app.add_systems(
        (
            tilemap::setup,
            ui::cursor::setup,
            ui::hp::setup,
            character::setup,
        )
            .in_schedule(OnEnter(AppState::InGame)),
    );
    app.add_systems(
        (
            character::update_character_sprite,
            character::play_character_sound,
            character::character_move,
            character::update_gun_direction,
            res::update_actor,
            ui::cursor::update,
            ui::hp::update,
        )
            .in_set(OnUpdate(AppState::InGame)),
    );

    app.run();
}

pub const CAMERA_FAR: f32 = 10.0;

fn setup_camera(mut c: Commands) {
    c.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            far: 10000.0,
            ..default()
        }
        .into(),
        // TODO 还不确定镜头的合适参数
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR).with_scale(Vec3::splat(0.04)),
        ..default()
    });
}
