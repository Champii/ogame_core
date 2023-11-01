use std::time::{SystemTime, UNIX_EPOCH};

use crate::{building_type::BuildingType, error::*, planet::Planet, protocol::Protocol};

pub struct Game {
    player_id: usize,
    planets: Vec<Planet>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_id: 0,
            planets: vec![],
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        for planet in &mut self.planets {
            planet.tick(now)?;
        }

        Ok(())
    }

    pub fn process_message(&mut self, msg: Protocol) -> Result<()> {
        self.tick()?;

        match msg {
            Protocol::UpgradeBuilding {
                planet_id,
                building_type,
            } => {
                self.upgrade_building(planet_id, building_type)?;
            }
            Protocol::Game(game) => {
                self.player_id = game.player_id;
                self.planets = game.planets;
            }
        }

        Ok(())
    }

    pub fn upgrade_building(
        &mut self,
        planet_id: usize,
        building_type: BuildingType,
    ) -> Result<()> {
        self.planets[planet_id].upgrade_building(building_type)?;

        Ok(())
    }
}
