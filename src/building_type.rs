use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::resources::Resources;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum BuildingType {
    Metal,
    Crystal,
    Deuterium,
}

impl Display for BuildingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingType::Metal => write!(f, "Metal"),
            BuildingType::Crystal => write!(f, "Crystal"),
            BuildingType::Deuterium => write!(f, "Deuterium"),
        }
    }
}

impl BuildingType {
    pub fn cost(&self, level: usize) -> Resources {
        match self {
            BuildingType::Metal => Resources {
                metal: 60.0 * 1.5f64.powi(level as i32 - 1) as f64,
                crystal: 15.0 * 1.5f64.powi(level as i32 - 1) as f64,
                deuterium: 0.0,
            },
            BuildingType::Crystal => Resources {
                metal: 48.0 * 1.6f64.powi(level as i32 - 1) as f64,
                crystal: 24.0 * 1.6f64.powi(level as i32 - 1) as f64,
                deuterium: 0.0,
            },
            BuildingType::Deuterium => Resources {
                metal: 225.0 * 1.5f64.powi(level as i32 - 1) as f64,
                crystal: 75.0 * 1.5f64.powi(level as i32 - 1) as f64,
                deuterium: 0.0,
            },
        }
    }

    pub fn build_time(&self, level: usize) -> usize {
        let cost = self.cost(level);

        let build_time =
            (cost.metal + cost.crystal) / (2500.0 * crate::UNIVERSE_SPEED as f64) * 3600.0;

        build_time.ceil() as usize
    }

    pub fn produced(&self, level: usize, ticks: usize) -> Resources {
        match self {
            BuildingType::Metal => Resources {
                metal: 30.0 * level as f64 * (1.1f64.powi(level as i32)) * ticks as f64 / 3600.0
                    * crate::UNIVERSE_SPEED as f64,
                crystal: 0.0,
                deuterium: 0.0,
            },
            BuildingType::Crystal => Resources {
                metal: 0.0,
                crystal: 20.0 * level as f64 * (1.1f64.powi(level as i32)) * ticks as f64 / 3600.0
                    * crate::UNIVERSE_SPEED as f64,
                deuterium: 0.0,
            },
            BuildingType::Deuterium => Resources {
                metal: 0.0,
                crystal: 0.0,
                deuterium: 10.0 * level as f64 * (1.1f64.powi(level as i32)) * ticks as f64
                    / 3600.0
                    * crate::UNIVERSE_SPEED as f64,
            },
        }
    }
}
