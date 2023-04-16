use bevy::prelude::*;

pub fn setup(
    mut c: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(32.0, 32.0)))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("art/initial_map.png")),
            perceptual_roughness: 0.8,
            metallic: 0.5,
            reflectance: 0.1,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 1.5),
        ..default()
    });
    c.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        ..default()
    });
    c.spawn(point_light(-15.0, 10.0, Color::rgba(1.0, 1.0, 1.0, 0.9)));
    c.spawn(point_light(15.0, -15.0, Color::rgba(0.8, 0.8, 0.0, 0.9)));
    c.spawn(point_light(-15.0, -15.0, Color::rgba(1.0, 0.5, 0.2, 0.9)));
}

fn point_light(x: f32, y: f32, color: Color) -> PointLightBundle {
    PointLightBundle {
        point_light: PointLight {
            color,
            intensity: 5000.0,
            range: 15.0,
            radius: 5.0,
            ..default()
        },
        transform: Transform::from_xyz(x, y, 10.0),
        ..default()
    }
}
