use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

use crate::cursor::ResCursor;
use crate::res::{Cache, ResActor, SCALE_RATIO};

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
    mut query: Query<(&mut Transform, &Bullet, Entity)>,
) {
    for (mut t, b, e) in query.iter_mut() {
        if (b.origin - t.translation.truncate()).length() > b.max_distance * SCALE_RATIO {
            c.entity(e).despawn_recursive();
        } else {
            let movement = time.delta_seconds() * b.velocity * b.speed * SCALE_RATIO;
            t.translation += movement.extend(-movement.y);
        }
    }
}
