use std::ops::SubAssign;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resources {
    pub metal: usize,
    pub crystal: usize,
    pub deuterium: usize,
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.metal -= rhs.metal;
        self.crystal -= rhs.crystal;
        self.deuterium -= rhs.deuterium;
    }
}
