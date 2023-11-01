use crate::{building_type::BuildingType, error::Result};

#[derive(Clone)]
struct BuildQueueItem {
    item: BuildingType,
    finish_date: usize,
}

#[derive(Clone)]
pub struct BuildQueue {
    items: Vec<BuildQueueItem>,
}

impl BuildQueue {
    pub fn push(&mut self, item: BuildingType, level: usize) {
        let finish_date =
            self.items.last().map(|item| item.finish_date).unwrap_or(0) + item.build_time(level);

        self.items.push(BuildQueueItem { item, finish_date });
    }

    pub fn tick(&mut self, now: usize) -> Result<Vec<BuildingType>> {
        let solved = self.get_solved_elements(now);
        self.items = self.items.drain(solved.len()..).collect();

        Ok(solved)
    }

    pub fn get_solved_elements(&self, now: usize) -> Vec<BuildingType> {
        self.items
            .iter()
            .filter(|item| item.finish_date <= now)
            .map(|item| item.item.clone())
            .collect()
    }
}
