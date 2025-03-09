use bevy::prelude::*;

use super::{Tile, CELL_SIZE, GRID};

pub fn background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = Mesh::from(Rectangle::from_size(CELL_SIZE));
    let color = Color::BLACK;

    let mesh = meshes.add(rect);
    let material = materials.add(color);

    commands.spawn((
        Name::new("Background"),
        Transform::from_xyz(CELL_SIZE.x / 2.0, CELL_SIZE.y / 2.0, -10.0),
        InheritedVisibility::default()
    ))
        .with_children(|cmd| {
        for i in 0..GRID.x {
            for j in 0..GRID.y {
                let position = Vec3::new(
                    i as f32 * CELL_SIZE.x,
                    j as f32 * CELL_SIZE.y,
                    0.0
                );
    
                cmd.spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_translation(position),
                    Tile([i, j])
                ));
            }
        }
    });

}