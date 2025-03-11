use bevy::prelude::*;

mod spawn;
mod interactions;

pub const GRID: UVec2 = UVec2::new(32, 32);
pub const CELL_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const GRID_TOP: f32 = 32.0 * 32.0;
pub const GRID_BOTTOM: f32 = 0.0;
pub const GRID_LEFT: f32 = 0.0;
pub const GRID_RIGHT: f32 = 32.0 * 32.0;

pub struct Background;

impl Plugin for Background {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn::background)
            .add_systems(Update, interactions::intersect);
    }
}

#[derive(Component)]
pub struct Tile([u32; 2]);