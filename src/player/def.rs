use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub color: Color
}

impl Player {
    pub fn create(name: String) -> Self {
        let color = Color::hsl(250.0, 1.0, 0.5);

        Self {
            name,
            color
        }
    }
}

#[derive(Component)]
pub struct LocalPlayer;