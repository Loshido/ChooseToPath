
use bevy::prelude::*;
use aeronet::io::web_time::{SystemTime, UNIX_EPOCH};
use def::{LocalPlayer, Player};

pub mod movements;
pub mod def;

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const FONT_SIZE: f32 = 12.0;
const SPAWN: Vec3 = Vec3::new(16.0 * 32.0, 16.0 * 32.0, 5.0);

pub fn spawn_player(
    mut command: Commands,
) {
    let name = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();

    let player = Player::create(
        format!("\"{}\"", name),
        (name % 100000) as f32
    );
    command.spawn((
        player.clone(),
        LocalPlayer,
        Transform::from_translation(SPAWN)
    ));

    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 0.5;
    command.spawn((
        Camera2d,
        projection,
        Transform::from_translation(SPAWN)
    ));
}

pub fn load_new_players(
    mut commands: Commands,
    players: Query<(Entity, &Player), Added<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let rect = Rectangle::from_size(PLAYER_SIZE);

    for (entity, player) in players.iter() {
        commands.entity(entity).insert((
            Name::new(player.name.clone()),
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(player.color))
        )).with_child((
            Text2d::new(player.name.clone()),
            TextFont::from_font_size(FONT_SIZE),
            Transform::from_xyz(0.0, PLAYER_SIZE.y * 1.25, 0.0)
        ));
    }
}