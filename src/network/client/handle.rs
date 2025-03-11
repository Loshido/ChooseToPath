use aeronet::{io::Session, transport::Transport};
use bevy::prelude::*;

use crate::{network::{JOIN_LANE, LEAVE_LANE, MAPS_LANE, MOVEMENTS_LANE}, player::def::Player};

pub fn recv(
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &mut Transform)>,
    mut clients: Query<
        (
            Entity,         // ..the entity ID
            &mut Transport, // ..and the transport layer access
        ),
        With<Session>
    >,
) {
    for (_, mut transport) in &mut clients {
        let transport = &mut *transport;

        for msg in transport.recv.msgs.drain() {
            match msg.lane {
                JOIN_LANE => {
                    match bincode::deserialize::<(Player, Vec2)>(&msg.payload) {
                        Ok((player, position)) => {
                            commands.spawn((
                                player,
                                Transform::from_xyz(position.x, position.y, 0.0),
                            ));
                        },
                        Err(_) => continue
                    }
                },
                LEAVE_LANE => {
                    match bincode::deserialize::<String>(&msg.payload) {
                        Ok(name) => {
                            for (entity, player, _) in players.iter() {
                                if player.name == name {
                                    commands.entity(entity).despawn();
                                }
                                break;
                            }
                        },
                        Err(_) => continue
                    }
                },
                MOVEMENTS_LANE => {
                    match bincode::deserialize::<Vec<(String, Vec2)>>(&msg.payload) {
                        Ok(positions) => {
                            'update: for (name, position) in positions.iter() {
                                for (_, player, mut transform) in players.iter_mut() {
                                    if name.to_string() == player.name {
                                        transform.translation.x = position.x;
                                        transform.translation.y = position.y;
                                    }
                                    break 'update;
                                }
                            }
                        },
                        Err(_) => continue
                    }
                },
                MAPS_LANE => {
                    match bincode::deserialize::<Vec<(String, [u32; 2])>>(&msg.payload) {
                        Ok(tiles) => {
                            println!("{} tiles to change", tiles.len());
                        },
                        Err(_) => continue
                    }
                },
                _ => ()
            }
            // _ = transport
            //     .send
            //     .push(msg.lane, Bytes::from(reply), web_time::Instant::now());
        }

        for _ in transport.recv.acks.drain() {
        }
    }
}