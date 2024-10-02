use std::cmp;

pub struct ConstantRandomizer {
    val: usize
}

impl ConstantRandomizer {
    pub fn new(val: usize) -> Self {
        ConstantRandomizer {val}
    }
}

impl hey_phineas::Randomizer for ConstantRandomizer {
    fn pick(&self, limit: usize) -> usize {
        cmp::min(self.val, limit - 1)
    }
}
