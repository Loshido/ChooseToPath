use aeronet::transport::lane::{LaneIndex, LaneKind};
use bevy::prelude::*;
use crate::player::def::Player;

pub mod client;
pub mod server;

const TARGET: &str = "wss://[::1]:25565";
const PORT: u16 = 25565;

const LANES: [LaneKind; 4] = [
    LaneKind::ReliableOrdered,
    LaneKind::ReliableOrdered,
    LaneKind::UnreliableUnordered,
    LaneKind::UnreliableUnordered
];

pub const JOIN_LANE: LaneIndex = LaneIndex::new(0);
pub const LEAVE_LANE: LaneIndex = LaneIndex::new(1);
pub const MOVEMENTS_LANE: LaneIndex = LaneIndex::new(2);
pub const MAPS_LANE: LaneIndex = LaneIndex::new(3);

// Server to client messages
pub enum OutPayload {
    Join(Player, Vec2),
    Leave(String),
    Move(Vec<(String, Vec2)>),
    Map(Vec<(String, [u32; 2])>)
}

// Client to server messages
pub enum InPayload {
    Join(Player, Vec2),
    Move(Vec2),
    Map([u32; 2])
}