use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::spawn_player;

mod player;
mod background;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            player::movements::apply,
            player::movements::follow,
            // gismos
        ))

        .add_plugins(background::Background)
        .run();
}