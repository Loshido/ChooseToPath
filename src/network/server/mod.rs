use bevy::prelude::*;
use aeronet::transport::AeronetTransportPlugin;
use aeronet_websocket::server::{ServerConfig, WebSocketServerPlugin, WebSocketServer};

use super::PORT;
mod handle;
mod events;
mod observers;

pub struct Network;

impl Plugin for Network {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AeronetTransportPlugin,
                WebSocketServerPlugin
            ))
            .add_systems(Startup, serve)
            .add_systems(Update, (
                handle::recv,
                events::on_move,
                events::on_join
            ))
            .add_observer(observers::on_connected)
            .add_observer(observers::on_opened)
            .add_observer(observers::on_disconnected);
    }
}

fn serve(mut commands: Commands) {
    let identity =
        aeronet_websocket::server::Identity::self_signed(["localhost", "127.0.0.1", "::1"])
            .expect("all given SANs should be valid DNS names");

    let config = ServerConfig::builder()
        .with_bind_default(PORT)
        .with_identity(identity);

    let mut server = commands.spawn_empty();
    server.queue(WebSocketServer::open(config));
}