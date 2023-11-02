use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resources {
    pub metal: usize,
    pub crystal: usize,
    pub deuterium: usize,
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            metal: 100000,
            crystal: 100000,
            deuterium: 100000,
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
