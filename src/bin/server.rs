use bevy::{log::LogPlugin, prelude::*};
use ChooseToPath::network;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            network::server::Network
        ))
        .run();
}