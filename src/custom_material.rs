use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::splat(20.0))))
            .into(),
        material: materials.add(ColorMaterial::from(asset_server.load("art/floor.png"))),
        transform: Transform::from_xyz(-50.0, 50.0, 0.0),
        ..default()
    });
}
