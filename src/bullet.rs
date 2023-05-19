use bevy::prelude::*;

use crate::character::CopGun;
use crate::cursor::ResCursor;
use crate::res::{Cache, ResActor};

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
    guns: Query<&GlobalTransform, With<CopGun>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let gun = guns.get_single().unwrap();
        let gun_pos = gun.translation();
        let actor_pos = actor.get_actual_pos();
        let cursor_pos = cursor.get_world_pos();
        c.spawn(PbrBundle {
            mesh: cache.get_bullet_mesh((5, 5)).clone(),
            material: cache.get_bullet_material("Budget Revolver").clone(),
            transform: Transform {
                translation: gun_pos,
                ..default()
            },
            ..default()
        })
        .insert(Bullet {
            origin: gun_pos.truncate(),
            velocity: (cursor_pos - actor_pos).truncate().normalize(),
            speed: 30.0,
            max_distance: 30.0,
        });
    }
}

pub fn bullet_move(
    mut c: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Bullet, Entity)>,
) {
    for (mut t, b, e) in query.iter_mut() {
        if (b.origin - t.translation.truncate()).length() > b.max_distance {
            c.entity(e).despawn_recursive();
        } else {
            let movement = time.delta_seconds() * b.velocity * b.speed;
            t.translation += movement.extend(-movement.y);
        }
    }
}
