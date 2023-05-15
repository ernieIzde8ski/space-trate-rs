use crate::api::schema;
use serde::{Deserialize, Serialize};

pub type ListShips = Vec<schema::Ship>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseShip {
    agent: schema::Agent,
    ship: schema::Ship,
    transaction: schema::MarketTransaction,
}

pub type GetShip = schema::Ship;
pub type GetShipCargo = schema::ShipCargo;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrbitShip {
    nav: schema::ShipNav,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipRefine {
    cargo: schema::ShipCargo,
    cooldown: schema::Cooldown,
    produced: schema::Produce,
    consumed: schema::Produce,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChart {
    chart: schema::Chart,
    waypoint: schema::Waypoint,
}

pub type GetShipCooldown = Option<schema::Cooldown>;
#[derive(Serialize, Deserialize, Debug)]
pub struct DockShip {
    nav: schema::ShipNav,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSurvey {
    cooldown: schema::Cooldown,
    surveys: schema::Survey,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractResources {
    cooldown: schema::Cooldown,
    extraction: schema::Extraction,
    cargo: schema::ShipCargo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JettisonCargo {
    cargo: schema::ShipCargo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JumpShip {
    cooldown: schema::Cooldown,
    nav: schema::ShipNav,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigateShip {
    fuel: schema::ShipFuel,
    nav: schema::ShipNav,
}

pub type PatchShipNav = schema::ShipNav;
pub type GetShipNav = schema::ShipNav;

#[derive(Serialize, Deserialize, Debug)]
pub struct WarpShip {
    fuel: schema::ShipFuel,
    nav: schema::ShipNav,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellCargo {
    agent: schema::Agent,
    cargo: schema::ShipCargo,
    transaction: schema::MarketTransaction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanSystems {
    cooldown: schema::Cooldown,
    systems: Vec<schema::ScannedSystem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanWaypoints {
    cooldown: schema::Cooldown,
    waypoints: Vec<schema::ScannedWaypoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanShips {
    cooldown: schema::Cooldown,
    ships: Vec<schema::ScannedShip>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefuelShip {
    agent: schema::Agent,
    fuel: schema::ShipFuel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseCargo {
    agent: schema::Agent,
    cargo: schema::ShipCargo,
    transaction: schema::MarketTransaction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferCargo {
    cargo: schema::ShipCargo,
}
