use bevy::prelude::*;
use aeronet::{io::{web_time::Instant, Session}, transport::Transport};

use crate::{network::{JOIN_LANE, MOVEMENTS_LANE}, player::def::{LocalPlayer, Player}};

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
pub fn on_move(
    mut transports: Query<&mut Transport, With<Session>>,
    players: Query<&Transform, (With<LocalPlayer>, Changed<Transform>, Without<Transport>)>
) {
    for player in players.iter() {
        let payload = bincode::serialize::<Vec2>(&player.translation.xy()).unwrap();
    
        for mut transport in transports.iter_mut() {
            let _ = transport.send.push(
                MOVEMENTS_LANE,
                payload.clone().into(),
                Instant::now()
            );
        }
    }
}