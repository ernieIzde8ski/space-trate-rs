use crate::api::schema;
use serde::{Deserialize, Serialize};

pub type ListContracts = Vec<schema::Contract>;
pub type GetContract = schema::Contract;

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptContract {
    agent: schema::Agent,
    contract: schema::Contract,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliverContract {
    contract: schema::Contract,
    cargo: schema::ShipCargo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FulfillContract {
    agent: schema::Agent,
    contract: schema::Contract,
}
