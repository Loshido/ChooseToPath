use bevy::prelude::*;
use aeronet::transport::{AeronetTransportPlugin, TransportConfig};
use aeronet_websocket::client::{ClientConfig, WebSocketClient, WebSocketClientPlugin};

use super::TARGET;

mod handle;
mod events;
mod observers;

pub struct Network;

impl Plugin for Network {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AeronetTransportPlugin,
                WebSocketClientPlugin
            ))
            .add_systems(Startup, connect)
            .add_systems(Update, (
                handle::recv,
                events::on_join,
            ))
            .add_observer(observers::on_connected)
            .add_observer(observers::on_disconnected)
            .add_observer(observers::on_connecting);
    }
}

fn connect(mut commands: Commands) {
    let config = ClientConfig::builder().with_no_cert_validation();
    let mut entity = commands.spawn(
        TransportConfig {
            max_memory_usage: 4 * 1024 * 1024,
            ..default()
        }
    );

    entity.queue(
        WebSocketClient::connect(config, TARGET)
    );
}