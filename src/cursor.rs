use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::res::SCALE_RATIO;

#[derive(Resource, Reflect, Default)]
pub struct ResCursor {
    /// actual bevy world coordinates
    pub pos: Vec3,
}

impl ResCursor {
    pub fn get_tilemap_pos(&self) -> [f32; 2] {
        [
            self.pos.x / SCALE_RATIO,
            -((self.pos.z - self.pos.y) / SCALE_RATIO) / SQRT_2,
        ]
    }
}

pub struct CursorDetectPlugin;

impl Plugin for CursorDetectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResCursor::default());
        app.add_system(detect_cursor_position);
        // #[cfg(debug_assertions)]
        {
            app.add_startup_system(setup_debug_cursor);
            app.add_system(update_debug_cursor);
        }
    }
}

pub fn detect_cursor_position(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<ResCursor>,
) {
    let primary_window = primary_window.get_single().unwrap();
    let (camera, global_transform) = camera_query.get_single().unwrap();
    if let Some(cursor_pos) = primary_window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(global_transform, cursor_pos) {
            cursor.pos = ray.origin;
        }
    }
}

#[derive(Component)]
struct DebugCursor;

fn setup_debug_cursor(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..default()
    })
    .insert(DebugCursor);
}

fn update_debug_cursor(
    cursor: Res<ResCursor>,
    mut query: Query<&mut Transform, With<DebugCursor>>,
) {
    for mut t in query.iter_mut() {
        t.translation.x = cursor.pos.x;
        t.translation.y = 0.0;
        t.translation.z = cursor.pos.z - cursor.pos.y;
    }
}
