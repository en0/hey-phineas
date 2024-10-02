use std::cmp;

pub struct ConstantRandomizer {
    val: usize
}

impl ConstantRandomizer {
    pub fn new(val: usize) -> Self {
        ConstantRandomizer {val}
    }
}

impl phineas_machine::Randomizer for ConstantRandomizer {
    fn pick(&self, limit: usize) -> usize {
        cmp::min(self.val, limit - 1)
    }
}
