use crate::api::schema;

macro_rules! json_deserialize_test {
    ($name:ident, $struct_type:ty, $file_path:expr) => {
        #[test]
        fn $name() {
            let string = std::fs::read_to_string($file_path).unwrap();
            serde_json::from_str::<$struct_type>(&string).unwrap();
        }
    };
}

json_deserialize_test!(test_market_schema, schema::Market, "schema-examples/Market.json");
json_deserialize_test!(test_ship_schema, schema::Ship, "schema-examples/Ship.json");
json_deserialize_test!(test_shipyard_schema, schema::Shipyard, "schema-examples/Shipyard.json");
json_deserialize_test!(test_waypoint_schema, schema::Waypoint, "schema-examples/Waypoint.json");
json_deserialize_test!(test_ship_nav_schema, schema::ShipNav, "schema-examples/ShipNav.json");
json_deserialize_test!(test_system_schema, schema::System, "schema-examples/System.json");
json_deserialize_test!(test_waypoint_trait_schema, schema::WaypointTrait, "schema-examples/WaypointTrait.json");
json_deserialize_test!(test_contract_schema, schema::Contract, "schema-examples/Contract.json");
json_deserialize_test!(test_survey_schema, schema::Survey, "schema-examples/Survey.json");
json_deserialize_test!(test_faction_schema, schema::Faction, "schema-examples/Faction.json");
json_deserialize_test!(test_scanned_waypoint_schema, schema::ScannedWaypoint, "schema-examples/ScannedWaypoint.json");
json_deserialize_test!(test_jump_gate_schema, schema::JumpGate, "schema-examples/JumpGate.json");
