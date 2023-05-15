use crate::api::schema;

pub type ListSystems = Vec<schema::System>;
pub type GetSystem = schema::System;
pub type ListWaypoints = Vec<schema::Waypoint>;
pub type GetWaypoint = schema::Waypoint;
pub type GetMarket = schema::Market;
pub type GetShipyard = schema::Shipyard;
pub type GetJumpGate = schema::JumpGate;
