use std::ops::{AddAssign, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    pub metal: f64,
    pub crystal: f64,
    pub deuterium: f64,
}

impl Resources {
    pub fn new(metal: f64, crystal: f64, deuterium: f64) -> Self {
        Resources {
            metal,
            crystal,
            deuterium,
        }
    }

    pub fn has_enough(&self, other: &Self) -> bool {
        self.metal >= other.metal
            && self.crystal >= other.crystal
            && self.deuterium >= other.deuterium
    }

    pub fn pay(&mut self, other: &Self) {
        self.metal -= other.metal;
        self.crystal -= other.crystal;
        self.deuterium -= other.deuterium;
    }

    pub fn gain(&mut self, other: &Self) {
        self.metal += other.metal;
        self.crystal += other.crystal;
        self.deuterium += other.deuterium;
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            metal: 5120.0,
            crystal: 1040.0,
            deuterium: 0.0,
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.metal -= rhs.metal;
        self.crystal -= rhs.crystal;
        self.deuterium -= rhs.deuterium;
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.metal += rhs.metal;
        self.crystal += rhs.crystal;
        self.deuterium += rhs.deuterium;
    }
}
