use std::f32::consts::PI;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

use crate::animation::OnceSpriteAnimation;
use crate::res::{Cache, ResActor, SCALE_RATIO};

pub struct CloudPuffEvent;

pub fn handle_cloud_puff_ev(
    mut c: Commands,
    mut evs: EventReader<CloudPuffEvent>,
    cache: Res<Cache>,
    actor: Res<ResActor>,
) {
    for _ in evs.iter() {
        let mut pos = actor.get_actual_pos();
        // 🫧气泡生成的位置应该比actor稍低一些，在脚附近
        pos.y -= 6.0 * SCALE_RATIO;
        pos.z += 6.0 * SCALE_RATIO;
        c.spawn((
            Name::from("Cloud Puff"),
            PbrBundle {
                mesh: cache.actor_caches.cloud_puff_mesh.clone(),
                material: cache.actor_caches.cloud_puff_materials[0].clone(),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                    .with_rotation(Quat::from_rotation_x(-PI / 4.0)),
                ..default()
            },
            OnceSpriteAnimation::new(0.1, cache.actor_caches.cloud_puff_materials.clone()),
            NotShadowCaster::default(),
        ));
    }
}
