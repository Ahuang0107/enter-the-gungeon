use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_resource::VertexFormat;
use bevy::sprite::MaterialMesh2dBundle;

/// 基于 bevy 的一个 High Level 的自定义顶点绘制2d内容的例子
pub fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    mesh.insert_attribute(
        MeshVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x2),
        VertexAttributeValues::from(vec![
            Vec2::new(-20.0, -20.0),
            Vec2::new(20.0, -20.0),
            Vec2::new(-20.0, 20.0),
            Vec2::new(20.0, 20.0),
        ]),
    );
    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_xyz(50.0, -50.0, 0.0),
        ..default()
    });
}
