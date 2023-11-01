use crate::{building_type::BuildingType, game::Game};

pub enum Protocol {
    // Server -> Client
    Game(Game),

    // Client -> Server
    UpgradeBuilding {
        planet_id: usize,
        building_type: BuildingType,
    },
}
