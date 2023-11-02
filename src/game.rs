use serde::{Deserialize, Serialize};
use web_time::UNIX_EPOCH;

use crate::{building_type::BuildingType, error::*, planet::Planet, protocol::Protocol};

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    player_id: usize,
    pub planets: Vec<Planet>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_id: 0,
            planets: vec![],
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let now = web_time::SystemTime::now()
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
        self.tick()?;

        self.planets[planet_id].upgrade_building(building_type)?;

        Ok(())
    }

    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }

    pub fn planets(&self) -> &Vec<Planet> {
        &self.planets
    }

    pub fn set_planets(&mut self, planets: Vec<Planet>) {
        self.planets = planets;
    }
}
