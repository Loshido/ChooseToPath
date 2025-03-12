use aeronet::{io::Session, transport::Transport};
use bevy::prelude::*;

use crate::{network::{JOIN_LANE, LEAVE_LANE, MAPS_LANE, MOVEMENTS_LANE}, player::def::{LocalPlayer, Player}};

pub fn recv(
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &mut Transform, Option<&LocalPlayer>)>,
    mut clients: Query<
        (
            Entity,         // ..the entity ID
            &mut Transport, // ..and the transport layer access
        ),
        With<Session>
    >,
) {
    let local_name = match players.iter().find(|p| p.3.is_some()) {
        Some(player) => player.1.name.clone(),
        _ => "localhost".to_string()
    };

    for (_, mut transport) in &mut clients {
        let transport = &mut *transport;

        for msg in transport.recv.msgs.drain() {
            match msg.lane {
                JOIN_LANE => {
                    match bincode::deserialize::<Vec<(Player, Vec2)>>(&msg.payload) {
                        Ok(players) => {
                            for (player, position) in players.iter() {
                                println!("Player {} joined", player.name);
                                if player.name == local_name {
                                    continue;
                                }
                                commands.spawn((
                                    player.clone(),
                                    Transform::from_xyz(position.x, position.y, 0.0),
                                ));
                            }
                        },
                        Err(_) => continue
                    }
                },
                LEAVE_LANE => {
                    match bincode::deserialize::<String>(&msg.payload) {
                        Ok(name) => {
                            println!("Player {} disconnected", name);
                            for (entity, player, _, _) in players.iter() {
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
                                for (_, player, mut transform, _) in players.iter_mut() {
                                    if name.to_string() == player.name && local_name != name.to_string() {
                                        transform.translation.x = position.x;
                                        transform.translation.y = position.y;
                                        break 'update;
                                    }
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