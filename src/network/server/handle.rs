use aeronet::{io::Session, transport::Transport};
use bevy::prelude::*;

use crate::{network::{JOIN_LANE, MAPS_LANE, MOVEMENTS_LANE}, player::def::Player};

pub fn recv(
    mut commands: Commands,
    mut clients: Query<
        (
            Entity,         // ..the entity ID
            &mut Transport, // ..and the transport layer access
            Option<&mut Transform>,
            Option<&Player>
        ),
        With<Session>
    >,
) {
    for (entity, mut transport, mut transform, _player) in clients.iter_mut() {
        let transport = &mut *transport;

        for msg in transport.recv.msgs.drain() {
            match msg.lane {
                JOIN_LANE => {
                    match bincode::deserialize::<(Player, Vec2)>(&msg.payload) {
                        Ok((player, position)) => {
                            commands.entity(entity)
                                .insert((
                                    player,
                                    Transform::from_xyz(position.x, position.y, 0.0)  
                                ));
                        },
                        Err(_) => continue
                    }
                },
                MOVEMENTS_LANE => {
                    match bincode::deserialize::<Vec2>(&msg.payload) {
                        Ok(position) => {
                            if let Some(ref mut transform) = transform {
                                transform.translation.x = position.x;
                                transform.translation.x = position.x;
                            }
                        },
                        Err(_) => continue
                    }
                },
                MAPS_LANE => {
                    match bincode::deserialize::<[u32; 2]>(&msg.payload) {
                        Ok(tile) => {
                            // if let Some(player) = player {
                            //     let payload = bincode::serialize::<>

                            //     let _ = transport.send.push(
                            //         MAPS_LANE, 
                            //         msg, 
                            //         Instant::now()
                            //     );
                            // }
                            println!("touched tile ({}:{})",
                                tile[0],
                                tile[1]
                            );
                        },
                        Err(_) => continue
                    }
                },
                _ => ()
            }
        }

        for _ in transport.recv.acks.drain() {
        }
    }
}