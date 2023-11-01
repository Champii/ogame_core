use std::collections::BTreeMap;

use crate::{build_queue::BuildQueue, building_type::BuildingType, error::*, resources::Resources};

#[derive(Clone)]
pub struct Planet {
    resources: Resources,
    buildings: BTreeMap<BuildingType, usize>,
    build_queue: BuildQueue,
    last_update: usize,
}

impl Planet {
    pub fn upgrade_building(&mut self, building_type: BuildingType) -> Result<()> {
        let current_level = *self.buildings.get(&building_type).unwrap_or(&0);

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

        self.last_update = now;

        Ok(())
    }
}
