use bevy::prelude::*;
use def::{LocalPlayer, Player};

pub mod movements;
pub mod def;

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const FONT_SIZE: f32 = 12.0;
const SPAWN: Vec3 = Vec3::new(32.0 * 32.0, 32.0 * 32.0, 5.0);

pub fn spawn_player(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let player = Player::create(
        String::from("localhost")
    );
    let rect = Rectangle::from_size(PLAYER_SIZE);

    command.spawn((
        player.clone(),
        LocalPlayer,
        Transform::from_translation(SPAWN),
        Mesh2d(meshes.add(rect)),
        MeshMaterial2d(materials.add(player.color))
    )).with_child((
        Text2d::new(player.name),
        TextFont::from_font_size(FONT_SIZE),
        Transform::from_xyz(0.0, PLAYER_SIZE.y * 1.25, 0.0)
    ));

    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 0.5;
    command.spawn((
        Camera2d,
        projection,
        Transform::from_translation(SPAWN)
    ));
}