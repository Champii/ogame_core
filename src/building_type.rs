use crate::resources::Resources;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum BuildingType {
    Metal,
    Crystal,
    Deuterium,
}

impl BuildingType {
    pub fn cost(&self, level: usize) -> Resources {
        match self {
            BuildingType::Metal => Resources {
                metal: 60 * 2usize.pow(level as u32),
                crystal: 15 * 2usize.pow(level as u32),
                deuterium: 0,
            },
            BuildingType::Crystal => Resources {
                metal: 48 * 2usize.pow(level as u32),
                crystal: 24 * 2usize.pow(level as u32),
                deuterium: 0,
            },
            BuildingType::Deuterium => Resources {
                metal: 225 * 2usize.pow(level as u32),
                crystal: 75 * 2usize.pow(level as u32),
                deuterium: 0,
            },
        }
    }

    pub fn build_time(&self, level: usize) -> usize {
        match self {
            BuildingType::Metal => 1 * 2usize.pow(level as u32),
            BuildingType::Crystal => 1 * 2usize.pow(level as u32),
            BuildingType::Deuterium => 1 * 2usize.pow(level as u32),
        }
    }

    pub fn produced(&self, level: usize, ticks: usize) -> Resources {
        match self {
            BuildingType::Metal => Resources {
                metal: 30 * level * ticks,
                crystal: 0,
                deuterium: 0,
            },
            BuildingType::Crystal => Resources {
                metal: 20 * level * ticks,
                crystal: 10 * level * ticks,
                deuterium: 0,
            },
            BuildingType::Deuterium => Resources {
                metal: 0,
                crystal: 0,
                deuterium: 10 * level * ticks,
            },
        }
    }
}