use crate::sprite_sheet::SpriteSheet;
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use std::f32::consts::PI;

#[derive(Bundle, Clone, Default)]
pub struct PbrSpriteBundle {
    pub sprite_sheet: SpriteSheet,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

pub fn update_base_texture(
    mut query: Query<(&SpriteSheet, &Handle<StandardMaterial>, &mut Transform)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (sprite_sheet, standard_material_handle, mut transform) in query.iter_mut() {
        match materials.get_mut(standard_material_handle) {
            Some(standard_material) => {
                if let Some(index) = sprite_sheet.index {
                    if let Some(texture) = sprite_sheet.texture_handles.get(&index) {
                        standard_material.base_color = Color::WHITE;
                        standard_material.base_color_texture = Some(texture.clone());
                        if sprite_sheet.flip_x {
                            standard_material.cull_mode = Some(Face::Front);
                            transform.rotation = Quat::from_rotation_y(-PI);
                        } else {
                            standard_material.cull_mode = Some(Face::Back);
                            transform.rotation = Quat::IDENTITY;
                        }
                    }
                }
            }
            None => {}
        }
    }
}
