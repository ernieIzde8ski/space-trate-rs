/*
The responses module contains information about what the different
endpoints return. It does not contain any information about how to
connect to those endpoints.
*/

use crate::api::schema;
use serde::{Deserialize, Serialize};

pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod systems;

pub mod agents {
    pub type MyAgentDetails = crate::api::schema::Agent;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    token: String,
    agent: schema::Agent,
    contract: schema::Contract,
    faction: schema::Faction,
    ship: schema::Ship,
}
