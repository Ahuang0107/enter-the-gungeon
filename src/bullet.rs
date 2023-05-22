use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

use world_generator::TileType;

use crate::cursor::ResCursor;
use crate::res::{Cache, ResActor, GRID_SIZE, SCALE_RATIO};

#[derive(Component)]
pub struct Bullet {
    origin: Vec2,
    velocity: Vec2,
    speed: f32,
    max_distance: f32,
}

pub fn fire_bullet(
    mut c: Commands,
    buttons: Res<Input<MouseButton>>,
    cache: Res<Cache>,
    cursor: Res<ResCursor>,
    actor: Res<ResActor>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(fire_offset) = actor.get_fire_offset() {
            let actor_pos = actor.get_actual_pos();

            let velocity = {
                let cursor_pos = cursor.get_world_pos();
                (cursor_pos - actor_pos).truncate().normalize()
            };

            c.spawn(PbrBundle {
                mesh: cache.get_bullet_mesh((5, 5)).clone(),
                material: cache.get_bullet_material("Budget Revolver").clone(),
                transform: Transform {
                    translation: actor_pos + fire_offset,
                    ..default()
                },
                ..default()
            })
            .insert(NotShadowCaster::default())
            .insert(Bullet {
                origin: (actor_pos + fire_offset).truncate(),
                velocity,
                speed: 600.0,
                max_distance: 600.0,
            });
        }
    }
}

pub fn bullet_move(
    mut c: Commands,
    time: Res<Time>,
    cache: Res<Cache>,
    mut query: Query<(&mut Transform, &Bullet, Entity)>,
) {
    let to_grid_pos = |pos: [f32; 2]| -> [i32; 2] {
        // TODO 目前不知道为什么整体偏移了(8,-24)
        let pos = [pos[0] + 8.0, pos[1] + 8.0];
        [
            (pos[0] / GRID_SIZE).floor() as i32,
            (pos[1] / GRID_SIZE).floor() as i32,
        ]
    };
    for (mut t, b, e) in query.iter_mut() {
        let pos = t.translation.truncate();
        if let Some(tile_type) =
            cache.levels[0].pos_tile(to_grid_pos([pos.x / SCALE_RATIO, pos.y / SCALE_RATIO]))
        {
            if tile_type == TileType::Roof || tile_type == TileType::Wall {
                c.entity(e).despawn_recursive();
                break;
            }
        }
        if (b.origin - pos).length() > b.max_distance * SCALE_RATIO {
            c.entity(e).despawn_recursive();
        } else {
            let movement = time.delta_seconds() * b.velocity * b.speed * SCALE_RATIO;
            t.translation += movement.extend(-movement.y);
        }
    }
}
