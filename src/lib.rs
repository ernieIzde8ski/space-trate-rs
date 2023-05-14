use reqwest;
pub mod responses;
pub mod schemas;
use responses::{ApiResult, RegistryResponse};

#[derive(Debug)]
pub struct SpaceTraderAgent {
    #[allow(dead_code)]
    client: reqwest::Client,
    token: String,
}

impl SpaceTraderAgent {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    #[tokio::main]
    pub async fn register(
        symbol: &str,
        faction: Option<&str>,
        client: Option<reqwest::Client>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        //! symbol should be a unique alphanumeric identifier, otherwise known as an agent
        //!
        //! faction can be any of "COSMIC", "VOID", "GALACTIC", or "QUANTUM" at the time
        //! of writing. COSMIC is said to be the default faction
        let client = client.unwrap_or_else(reqwest::Client::new);
        let params: [(&str, &str); 2] =
            [("symbol", symbol), ("faction", faction.unwrap_or("COSMIC"))];
        let result = client
            .post("https://api.spacetraders.io/v2/register")
            .header("Content-Type", "application/json")
            .form(&params)
            .send()
            .await?
            .json::<ApiResult<RegistryResponse>>()
            .await?
            .to_result()?;

        Ok(SpaceTraderAgent {
            client: client,
            token: result.token,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::schemas;

    macro_rules! json_deserialize_test {
        ($name:ident, $struct_type:ty, $file_path:expr) => {
            #[test]
            fn $name() {
                let string = std::fs::read_to_string($file_path).unwrap();
                serde_json::from_str::<$struct_type>(&string).unwrap();
            }
        };
    }

    json_deserialize_test!(test_market_schema, schemas::Market, "schema-examples/Market.json");
    json_deserialize_test!(test_ship_schema, schemas::Ship, "schema-examples/Ship.json");
    json_deserialize_test!(test_shipyard_schema, schemas::Shipyard, "schema-examples/Shipyard.json");
    json_deserialize_test!(test_waypoint_schema, schemas::Waypoint, "schema-examples/Waypoint.json");
    json_deserialize_test!(test_ship_nav_schema, schemas::ShipNav, "schema-examples/ShipNav.json");
    json_deserialize_test!(test_system_schema, schemas::System, "schema-examples/System.json");
    json_deserialize_test!(test_waypoint_trait_schema, schemas::WaypointTrait, "schema-examples/WaypointTrait.json");
    json_deserialize_test!(test_contract_schema, schemas::Contract, "schema-examples/Contract.json");
    json_deserialize_test!(test_survey_schema, schemas::Survey, "schema-examples/Survey.json");
    json_deserialize_test!(test_faction_schema, schemas::Faction, "schema-examples/Faction.json");
    json_deserialize_test!(test_scanned_waypoint_schema, schemas::ScannedWaypoint, "schema-examples/ScannedWaypoint.json");
    json_deserialize_test!(test_jump_gate_schema, schemas::JumpGate, "schema-examples/JumpGate.json");
}
