use bevy::{prelude::*, window::PrimaryWindow};
use super::def::LocalPlayer;
use crate::background::{GRID_TOP, GRID_BOTTOM, GRID_LEFT, GRID_RIGHT};

const SPEED: f32 = 100.0; // pixel/s
const CAMERA_SPEED: f32 = 1.0;

pub fn apply(
    time: Res<Time>,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, (With<LocalPlayer>, Without<Camera>)>
) {
    let mut vel = Vec2::ZERO;

    if key.pressed(KeyCode::KeyW) {
        vel.y += SPEED;
    }
    if key.pressed(KeyCode::KeyS) {
        vel.y -= SPEED;
    }
    if key.pressed(KeyCode::KeyD) {
        vel.x += SPEED;
    }
    if key.pressed(KeyCode::KeyA) {
        vel.x -= SPEED;
    }

    if vel.distance(Vec2::ZERO) < SPEED * 0.25 {
        return
    }
    player.translation.x += vel.x * time.delta_secs();
    player.translation.y += vel.y * time.delta_secs();
}

pub fn follow(
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    player: Single<&Transform, (With<LocalPlayer>, Without<Camera>)>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<LocalPlayer>)>
) {
    if player.translation.x < 0.0 && player.translation.y < 0.0 {
        return;
    }

    let height = window.resolution.height() / 4.0;
    let width = window.resolution.width() / 4.0;

    let mut computed_translation = camera.translation + 
        (player.translation - camera.translation) * time.delta_secs() * CAMERA_SPEED;
    if computed_translation.x - width < GRID_LEFT ||
        computed_translation.x + width > GRID_RIGHT {
        computed_translation.x =  camera.translation.x;
    }
    if computed_translation.y - height < GRID_BOTTOM ||
    computed_translation.y + height > GRID_TOP {
        computed_translation.y = camera.translation.y;
    }

    camera.translation = computed_translation;
}