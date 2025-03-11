use bevy::prelude::*;
use aeronet::{transport::Transport, io::web_time::Instant};

use crate::{network::JOIN_LANE, player::def::{LocalPlayer, Player}};

pub fn on_join(
    mut transports: Query<&mut Transport, Added<Transport>>,
    player: Single<(&Player, &Transform), With<LocalPlayer>>
) {
    let payload = bincode::serialize::<(Player, Vec2)>(&(
        player.0.clone(),
        player.1.translation.xy()
    )).expect("invalid player");

    for mut transport in transports.iter_mut() {
        let _ = transport.send.push(
            JOIN_LANE,
            payload.clone().into(),
            Instant::now()
        );
    }
}