use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{build_queue::BuildQueue, building_type::BuildingType, error::*, resources::Resources};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Planet {
    id: usize,
    resources: Resources,
    pub buildings: BTreeMap<BuildingType, usize>,
    pub build_queue: BuildQueue,
    last_update: usize,
}

impl Planet {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn new(id: usize) -> Self {
        Planet {
            id,
            resources: Resources::default(),
            buildings: vec![
                (BuildingType::Metal, 0),
                (BuildingType::Crystal, 0),
                (BuildingType::Deuterium, 0),
            ]
            .into_iter()
            .collect(),
            build_queue: BuildQueue::default(),
            last_update: web_time::SystemTime::now()
                .duration_since(web_time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        }
    }

    pub fn resources(&self) -> Resources {
        self.resources.clone()
    }

    pub fn upgrade_building(&mut self, building_type: BuildingType) -> Result<()> {
        let mut current_level = *self.buildings.get(&building_type).unwrap_or(&0);

        for building in self.build_queue.items.iter() {
            if building.r#type == building_type {
                current_level += 1;
            }
        }

        let cost = building_type.cost(current_level);

        self.pay(cost)?;

        self.build_queue.push(building_type, current_level + 1);

        Ok(())
    }

    pub fn has_enough(&self, cost: &Resources) -> bool {
        self.resources >= *cost
    }

    pub fn pay(&mut self, cost: Resources) -> Result<()> {
        if !self.has_enough(&cost) {
            return Err(Error::NotEnoughResources);
        }

        self.resources -= cost;

        Ok(())
    }

    pub fn tick(&mut self, now: usize) -> Result<()> {
        let buildings_update = self.build_queue.tick(now)?;

        for building in buildings_update {
            let current_level = self.buildings.get_mut(&building).unwrap(); // Should always exist
            *current_level += 1;
        }

        for (building, level) in &self.buildings {
            let produced = building.produced(*level, now - self.last_update);

            self.resources += produced;
        }

        self.last_update = now;

        Ok(())
    }

    pub fn building_level(&self, building_type: BuildingType) -> usize {
        *self.buildings.get(&building_type).unwrap_or(&0)
    }

    pub fn last_update(&self) -> usize {
        self.last_update
    }
}
