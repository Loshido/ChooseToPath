use aeronet::{io::{connection::Disconnected, Session, SessionEndpoint, web_time::Instant}, transport::Transport};
use bevy::prelude::*;

use crate::network::LANES;

pub fn on_connecting(_trigger: Trigger<OnAdd, SessionEndpoint>) {
    println!("Connecting");
}

pub fn on_disconnected(_trigger: Trigger<Disconnected>) {
    println!("Disconnected");
}

pub fn on_connected(
    trigger: Trigger<OnAdd, Session>,
    sessions: Query<&Session>,
    mut commands: Commands
) {
    let entity = trigger.entity();
    let session = sessions
        .get(entity)
        .unwrap();

    // Once the `Session` is added, we can make a `Transport`
    // and use messages.
    let transport = Transport::new(
        session,
        LANES,
        LANES,
        Instant::now(),
    )
    .expect("packet MTU should be large enough to support transport");

    commands.entity(entity).insert(transport);

    println!("Connected");
}