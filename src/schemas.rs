use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn parse_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map_err(|e| serde::de::Error::custom(format!("{}", e)))
        .map(|dt| dt.into())
}
fn parse_optional_date_time<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => DateTime::parse_from_rfc3339(&s)
            .map_err(|e| serde::de::Error::custom(format!("{}", e)))
            .map(|dt| Some(dt.into())),
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    #[serde(rename = "accountId")]
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i32,
    // The number of credits the agent has available. Credits can be negative if funds have been overdrawn.
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    waypoint_symbol: Option<String>,
    submitted_by: Option<String>,
    #[serde(deserialize_with = "parse_optional_date_time")]
    submitted_on: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedSystem {
    symbol: String,
    sector_symbol: String,
    #[serde(rename = "type")]
    kind: SystemType,
    x: i32,
    y: i32,
    distance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    id: String,
    faction_symbol: String,
    #[serde(rename = "type")]
    kind: ContractType,
    terms: ContractTerms,
    accepted: bool,
    fulfilled: bool,
    #[serde(deserialize_with = "parse_date_time")]
    expiration: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliverGood {
    trade_symbol: String,
    destination_symbol: String,
    units_required: i32,
    units_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractPayment {
    on_accepted: i32,
    on_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractTerms {
    #[serde(deserialize_with = "parse_date_time")]
    deadline: DateTime<Utc>,
    payment: ContractPayment,
    deliver: Vec<ContractDeliverGood>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    ship_symbol: String,
    total_seconds: i32,
    remaining_seconds: i32,
    #[serde(deserialize_with = "parse_date_time")]
    expiration: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    ship_symbol: String,
    #[serde(rename = "yield")]
    yield_amount: ExtractionYield,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractionYield {
    symbol: String,
    units: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    symbol: String,
    name: String,
    description: String,
    headquarters: String,
    traits: Vec<FactionTrait>,
}

pub type FactionTrait = TypedSymbolic<FactionTraitSymbols>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionTraitSymbols {
    Bureaucratic,
    Secretive,
    Capitalistic,
    Industrious,
    Peaceful,
    Distrustful,
    Welcoming,
    Smugglers,
    Scavengers,
    Rebellious,
    Exiles,
    Pirates,
    Raiders,
    Clan,
    Guild,
    Dominion,
    Fringe,
    Forsaken,
    Isolated,
    Localized,
    Established,
    Notable,
    Dominant,
    Inescapable,
    Innovative,
    Bold,
    Visionary,
    Curious,
    Daring,
    Exploratory,
    Resourceful,
    Flexible,
    Cooperative,
    United,
    Strategic,
    Intelligent,
    ResearchFocused,
    Collaborative,
    Progressive,
    Militaristic,
    TechnologicallyAdvanced,
    Aggressive,
    Imperialistic,
    TreasureHunters,
    Dexterous,
    Unpredictable,
    Brutal,
    Fleeting,
    Adaptable,
    SelfSufficient,
    Defensive,
    Proud,
    Diverse,
    Independent,
    SelfInterested,
    Fragmented,
    Commercial,
    FreeMarkets,
    Entrepreneurial,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JumpGate {
    jump_range: i32,
    faction_symbol: String,
    connected_systems: Vec<ConnectedSystem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    symbol: String,
    exports: Vec<TradeGood>,
    imports: Vec<TradeGood>,
    exchange: Vec<TradeGood>,
    transactions: Vec<MarketTransaction>,
    trade_goods: Vec<MarketTradeGood>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketTradeGood {
    symbol: String,
    trade_volume: u32,
    supply: String,
    purchase_price: u32,
    sell_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketTransaction {
    waypoint_symbol: String,
    ship_symbol: String,
    trade_symbol: String,
    #[serde(rename = "type")]
    kind: MarketTransactionTypes,
    units: u32,
    price_per_unit: u32,
    total_price: u32,
    #[serde(deserialize_with = "parse_date_time")]
    timestamp: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketTransactionTypes {
    Purchase,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    total: i32,
    page: i32,
    limit: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScannedShip {
    symbol: String,
    registration: ShipRegistration,
    nav: ShipNav,
    frame: Symbolic,
    reactor: Symbolic,
    engine: Symbolic,
    mounts: Vec<Symbolic>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedSystem {
    symbol: String,
    sector_symbol: String,
    #[serde(rename = "type")]
    kind: SystemType,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScannedWaypoint {
    symbol: String,
    #[serde(rename = "type")]
    kind: WaypointType,
    system_symbol: String,
    x: i32,
    y: i32,
    orbitals: Vec<Symbolic>,
    faction: Symbolic,
    traits: Vec<WaypointTrait>,
    chart: Option<Chart>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    symbol: String,
    registration: ShipRegistration,
    nav: ShipNav,
    crew: ShipCrew,
    frame: ShipFrame,
    reactor: ShipReactor,
    engine: ShipEngine,
    modules: Vec<ShipModule>,
    mounts: Vec<ShipMount>,
    cargo: ShipCargo,
    fuel: ShipFuel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCargo {
    capacity: u32,
    units: u32,
    inventory: Vec<ShipCargoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCargoItem {
    symbol: String,
    name: String,
    description: String,
    units: u32,
}

pub type ShipCondition = u8;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCrew {
    current: i32,
    required: i32,
    capacity: i32,
    rotation: ShipCrewRotation,
    morale: u8,
    wages: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipCrewRotation {
    Strict,
    Relaxed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipEngine {
    symbol: ShipEngineSymbols,
    name: String,
    description: String,
    condition: Option<ShipCondition>,
    speed: u32,
    requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipEngineSymbols {
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFrame {
    symbol: ShipFrameSymbols,
    name: String,
    description: String,
    condition: Option<ShipCondition>,
    module_slots: i32,
    mounting_points: i32,
    fuel_capacity: i32,
    requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipFrameSymbols {
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipFuel {
    current: u32,
    capacity: u32,
    consumed: Option<ShipFuelConsumption>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipFuelConsumption {
    amount: u32,
    #[serde(deserialize_with = "parse_date_time")]
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipModule {
    symbol: ShipModuleSymbols,
    capacity: Option<u32>,
    range: Option<u32>,
    name: String,
    description: Option<String>,
    requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipModuleSymbols {
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveIi,
    ModuleJumpDriveIii,
    ModuleWarpDriveI,
    ModuleWarpDriveIi,
    ModuleWarpDriveIii,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorIi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipMount {
    symbol: ShipMountSymbols,
    name: String,
    description: Option<String>,
    strength: Option<u32>,
    deposits: Option<Vec<Deposits>>,
    requirements: ShipRequirements,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipMountSymbols {
    MountGasSiphonI,
    MountGasSiphonIi,
    MountGasSiphonIii,
    MountSurveyorI,
    MountSurveyorIi,
    MountSurveyorIii,
    MountSensorArrayI,
    MountSensorArrayIi,
    MountSensorArrayIii,
    MountMiningLaserI,
    MountMiningLaserIi,
    MountMiningLaserIii,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNav {
    system_symbol: String,
    waypoint_symbol: String,
    route: ShipNavRoute,
    status: ShipNavStatus,
    flight_mode: ShipNavFlightMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavFlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}
/// default: ShipNavFlightMode::CRUISE

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRoute {
    destination: ShipNavRouteWaypoint,
    departure: ShipNavRouteWaypoint,

    #[serde(deserialize_with = "parse_date_time")]
    departure_time: DateTime<Utc>,

    #[serde(deserialize_with = "parse_date_time")]
    arrival: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRouteWaypoint {
    symbol: String,
    #[serde(rename = "type")]
    kind: WaypointType,
    system_symbol: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipNavStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    symbol: ShipReactorSymbols,
    name: String,
    description: String,
    condition: Option<ShipCondition>,
    power_output: u32,
    requirements: ShipRequirements,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipReactorSymbols {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipRegistration {
    name: String,
    faction_symbol: String,
    role: ShipRole,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipRequirements {
    power: Option<i32>,
    crew: Option<i32>,
    slots: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipRole {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    ShipProbe,
    ShipMiningDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipTypeObject {
    #[serde(rename = "type")]
    kind: ShipType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    symbol: String,
    ship_types: Vec<ShipTypeObject>,
    transactions: Vec<ShipyardTransaction>,
    ships: Vec<ShipyardShip>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    kind: Option<ShipType>,
    name: String,
    description: String,
    purchase_price: i32,
    frame: ShipFrame,
    reactor: ShipReactor,
    engine: ShipEngine,
    modules: Vec<ShipModule>,
    mounts: Vec<ShipMount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardTransaction {
    waypoint_symbol: String,
    ship_symbol: String,
    price: u32,
    agent_symbol: String,

    #[serde(deserialize_with = "parse_date_time")]
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Survey {
    signature: String,
    symbol: String,
    deposits: Vec<SurveyDeposit>,
}

pub type SurveyDeposit = Symbolic;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct System {
    symbol: String,
    sector_symbol: String,
    #[serde(rename = "type")]
    kind: SystemType,
    x: i32,
    y: i32,
    waypoints: Vec<SystemWaypoint>,
    factions: Vec<SystemFaction>,
}

pub type SystemFaction = Symbolic;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemType {
    NeutronStar,
    RedStar,
    OrangeStar,
    BlueStar,
    YoungStar,
    WhiteDwarf,
    BlackHole,
    Hypergiant,
    Nebula,
    Unstable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemWaypoint {
    symbol: String,
    #[serde(rename = "type")]
    kind: WaypointType,
    x: i32,
    y: i32,
}

pub type TradeGood = TypedSymbolic<TradeSymbol>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeSymbol {
    PreciousStones,
    QuartzSand,
    SiliconCrystals,
    AmmoniaIce,
    LiquidHydrogen,
    LiquidNitrogen,
    IceWater,
    ExoticMatter,
    AdvancedCircuitry,
    GravitonEmitters,
    Iron,
    IronOre,
    Copper,
    CopperOre,
    Aluminum,
    AluminumOre,
    Silver,
    SilverOre,
    Gold,
    GoldOre,
    Platinum,
    PlatinumOre,
    Diamonds,
    Uranite,
    UraniteOre,
    Meritium,
    MeritiumOre,
    Hydrocarbon,
    Antimatter,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    AssaultRifles,
    MilitaryEquipment,
    Explosives,
    LabInstruments,
    Ammunition,
    Electronics,
    ShipPlating,
    Equipment,
    Fuel,
    Medicine,
    Drugs,
    Clothing,
    Microprocessors,
    Plastics,
    Polynucleotides,
    Biocomposites,
    Nanobots,
    AiMainframes,
    QuantumDrives,
    RoboticDrones,
    CyberImplants,
    GeneTherapeutics,
    NeuralChips,
    MoodRegulators,
    ViralAgents,
    MicroFusionGenerators,
    Supergrains,
    LaserRifles,
    Holographics,
    ShipSalvage,
    RelicTech,
    NovelLifeforms,
    BotanicalSpecimens,
    CulturalArtifacts,
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveIi,
    EngineHyperDriveI,
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveIi,
    ModuleJumpDriveIii,
    ModuleWarpDriveI,
    ModuleWarpDriveIi,
    ModuleWarpDriveIii,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorIi,
    MountGasSiphonI,
    MountGasSiphonIi,
    MountGasSiphonIii,
    MountSurveyorI,
    MountSurveyorIi,
    MountSurveyorIii,
    MountSensorArrayI,
    MountSensorArrayIi,
    MountSensorArrayIii,
    MountMiningLaserI,
    MountMiningLaserIi,
    MountMiningLaserIii,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    symbol: String,
    #[serde(rename = "type")]
    kind: WaypointType,
    system_symbol: String,
    x: i32,
    y: i32,
    orbitals: Vec<WaypointOrbital>,
    faction: WaypointFaction,
    traits: Vec<WaypointTrait>,
    chart: Option<Chart>,
}

pub type WaypointFaction = Symbolic;
pub type WaypointOrbital = Symbolic;
pub type WaypointTrait = TypedSymbolic<WaypointTraitSymbol>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTraitSymbol {
    Uncharted,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Stripped,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Nebula,
    DebrisField,
    GravityWell,
}

/* types not declared by the documentation go here */

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbolic {
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypedSymbolic<T> {
    symbol: T,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Deposits {
    QuartzSand,
    SiliconCrystals,
    PreciousStones,
    IceWater,
    AmmoniaIce,
    IronOre,
    CopperOre,
    SilverOre,
    AluminumOre,
    GoldOre,
    PlatinumOre,
    Diamonds,
    UraniteOre,
    MeritiumOre,
}
