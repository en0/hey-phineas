use rand::Rng;

pub trait Randomizer {
    fn pick(&self, limit: usize) -> usize;
}

pub struct BuiltInRandomizer;
impl Randomizer for BuiltInRandomizer {
    fn pick(&self, limit: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..limit)
    }
}
