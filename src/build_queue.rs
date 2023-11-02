use crate::{building_type::BuildingType, error::Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildQueueItem {
    pub r#type: BuildingType,
    pub finish_date: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BuildQueue {
    pub items: Vec<BuildQueueItem>,
}

impl BuildQueue {
    pub fn push(&mut self, item: BuildingType, level: usize) {
        let finish_date = self.items.last().map(|item| item.finish_date).unwrap_or(
            web_time::SystemTime::now()
                .duration_since(web_time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        ) + item.build_time(level);

        self.items.push(BuildQueueItem {
            r#type: item,
            finish_date,
        });
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
            .map(|item| item.r#type.clone())
            .collect()
    }
}
