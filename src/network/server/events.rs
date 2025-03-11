use aeronet::{io::{Session, web_time::Instant}, transport::Transport};
use bevy::prelude::*;

use crate::{network::{JOIN_LANE, MOVEMENTS_LANE}, player::def::Player};

pub fn on_join(
    mut transports: Query<(&mut Transport, &Player), With<Session>>,
    players: Query<(&Player, &Transform), Added<Player>>
) {
    if players.is_empty() {
        return;
    }

    for (mut transport, remote_player) in transports.iter_mut() {
        for player in players.iter() {
            if remote_player.name == player.0.name {
                continue;
            }
            
            let payload = bincode::serialize::<(Player, Vec2)>(
                &(
                    player.0.clone(),
                    player.1.translation.xy()
                )
            );

            let _ = transport.send.push(
                JOIN_LANE,
                payload.unwrap().into(), 
                Instant::now()
            );
        }
    }
}

pub fn on_move(
    mut transports: Query<&mut Transport, With<Session>>,
    players: Query<(&Player, &Transform), Changed<Player>>
) {
    if players.is_empty() {
        return;
    }

    let players: Vec<(String, Vec2)> = players.iter()
        .map(|ply| (
            ply.0.name.clone(),
            ply.1.translation.xy()
        ))
        .collect();

    for mut transport in transports.iter_mut() {
        let payload = bincode::serialize::<Vec<(String, Vec2)>>(&players);

        let _ = transport.send.push(
            MOVEMENTS_LANE,
            payload.unwrap().into(), 
            Instant::now()
        );
    }
}