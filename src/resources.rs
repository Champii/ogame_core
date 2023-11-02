use std::{
    cmp::Ordering,
    ops::{AddAssign, SubAssign},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources {
    pub metal: usize,
    pub crystal: usize,
    pub deuterium: usize,
}

impl Ord for Resources {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.metal.cmp(&other.metal) {
            Ordering::Equal | Ordering::Greater => match self.crystal.cmp(&other.crystal) {
                Ordering::Equal | Ordering::Greater => self.deuterium.cmp(&other.deuterium),
                other => other,
            },
            other => other,
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.metal
            .partial_cmp(&other.metal)
            .and_then(|metal| match metal {
                Ordering::Equal | Ordering::Greater => self
                    .crystal
                    .partial_cmp(&other.crystal)
                    .and_then(|crystal| match crystal {
                        Ordering::Equal | Ordering::Greater => {
                            self.deuterium.partial_cmp(&other.deuterium)
                        }
                        other => Some(other),
                    }),
                other => Some(other),
            })
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            metal: 120,
            crystal: 40,
            deuterium: 0,
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
