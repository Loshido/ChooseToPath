use aeronet::{io::Session, transport::Transport};
use bevy::{asset::transformer, prelude::*};

use crate::{network::{JOIN_LANE, MAPS_LANE, MOVEMENTS_LANE}, player::{self, def::Player}};

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
                            println!("joined {:?}, {:?}", player.name, position);
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
                                transform.translation.y = position.y;
                            }
                        },
                        Err(_) => {
                            println!("failed to deserialize movements");
                            continue
                        }
                    }
                },
                MAPS_LANE => {
                    match bincode::deserialize::<[u32; 2]>(&msg.payload) {
                        Ok(tile) => {
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