use aeronet::{io::{connection::{Disconnected, LocalAddr}, server::Server, web_time::Instant, Session}, transport::Transport};
use bevy::prelude::*;

use crate::{network::{LANES, LEAVE_LANE}, player::def::Player};

// Observe state change events using `Trigger`s
pub fn on_opened(trigger: Trigger<OnAdd, Server>, servers: Query<&LocalAddr>) {
    let server = trigger.entity();
    let local_addr = servers
        .get(server)
        .expect("opened server should have a binding socket `LocalAddr`");
    info!("{server} opened on {}", **local_addr);
}

pub fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    sessions: Query<&Session>,
    mut commands: Commands,
) {
    let client = trigger.entity();
    let session = sessions
        .get(client)
        .expect("we are adding this component to this entity");

    info!("{client} connected");

    let transport = Transport::new(
        session,
        LANES,
        LANES,
        Instant::now(),
    )
    .expect("packet MTU should be large enough to support transport");

    commands.entity(client).insert(transport);
}

pub fn on_disconnected(trigger: Trigger<Disconnected>,
    mut players: Query<(&Player, Option<&mut Transport>)>) {
    let client = trigger.entity();
    let name = match players.get_mut(client) {
        Ok(player) => &player.0.name.clone(),
        Err(_) => return
    };

    for (player, mut transport) in players.iter_mut() {
        if player.name == name.clone() {
            continue;
        }

        if let Some(ref mut transport) = transport {
            let payload = bincode::serialize::<String>(&name);

            let _ = transport.send.push(
                LEAVE_LANE, 
                payload.unwrap().into(), 
                Instant::now()
            );
        }
    }

    // players.get(client);
    println!("{} disconnected", client.index());
}