use bevy::prelude::*;
use crate::player::def::LocalPlayer;
use super::{Tile, CELL_SIZE, GRID};

pub fn intersect(
    mut tiles: Query<(&Tile, &mut MeshMaterial2d<ColorMaterial>), Without<LocalPlayer>>,
    players: Query<(&Transform, &MeshMaterial2d<ColorMaterial>), (Changed<Transform>, With<LocalPlayer>)>
) {
    let player = match players.get_single() {
        Ok(pl) => pl,
        _ => return
    };
    let pos = player.0.translation;
    if pos.x < 0.0 || pos.y < 0.0 {
        return;
    }

    let i = (pos.x / CELL_SIZE.x) as u32;
    let j = (pos.y / CELL_SIZE.y) as u32;

    if i > GRID.x || j > GRID.y {
        return;
    }

    let mut tile = match tiles
        .iter_mut()
        .filter(|t| t.0.0 == [i, j])
        .next() {
        Some(tile) => tile,
        _ => {
            // out of range ? 
            return;
        }
    };

    tile.1.0 = player.1.0.clone();
}