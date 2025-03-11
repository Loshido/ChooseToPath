use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ChooseToPath::{background, network, player::{load_new_players, movements, spawn_player}};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (
            movements::apply,
            movements::follow,
            load_new_players
            // gismos
        ))

        .add_plugins((
            background::Background,
            network::client::Network
        ))
        .run();
}