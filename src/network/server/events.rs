use std::ops::Deref;

use aeronet::{io::web_time::Instant, transport::Transport};
use bevy::prelude::*;
use bincode::serialize;

use crate::{network::{JOIN_LANE, MOVEMENTS_LANE}, player::def::Player};

pub fn on_change(
    mut players: Query<(&mut Transport, Ref<Player>, Ref<Transform>)>
) {
    let mut players_list: Vec<(Player, Vec2)> = Vec::new();
    let mut players_moving: Vec<(String, Vec2)> = Vec::new();
    let mut players_incomming: Vec<(Player, Vec2)> = Vec::new();
    for (_, player, transform) in players.iter() {
        if player.is_added() {
            players_incomming.push((
                player.clone(),
                transform.translation.xy()
            ));
        } else if transform.is_changed() {
            players_moving.push((
                player.name.clone(),
                transform.translation.xy()
            ))
        }
    }

    if !players_incomming.is_empty() {
        players_list = players.iter()
            .map(|p| (p.1.deref().clone(), p.2.translation.xy()))
            .collect();
    }

    let join_payload = serialize::<Vec<(Player, Vec2)>>(&players_incomming)
        .unwrap();
    let move_payload = serialize::<Vec<(String, Vec2)>>(&players_moving)
        .unwrap();
    let players_payload = serialize::<Vec<(Player, Vec2)>>(&players_list)
        .unwrap();

    for (mut transport, ply, _) in players.iter_mut() {
        if let Some(_) = players_incomming.iter().find(|p| p.0.name == ply.name) {
            let _ = transport.send.push(
                JOIN_LANE, 
                players_payload.clone().into(), 
                Instant::now()
            );
        }

        if !players_incomming.is_empty() {
            let _ = transport.send.push(
                JOIN_LANE, 
                join_payload.clone().into(), 
                Instant::now()
            );
        }
        if !players_moving.is_empty() {
            let _ = transport.send.push(
                MOVEMENTS_LANE, 
                move_payload.clone().into(), 
                Instant::now()
            );
        }
    }
}