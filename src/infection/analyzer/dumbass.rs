use crate::infection::analyzer::Analyzer;
use crate::infection::field::*;
use rand::Rng;

pub struct Dumbass;
impl Analyzer for Dumbass {
    fn heuristic(&self, _player_id: u8, _field: Field) -> f32 {
        rand::thread_rng().gen_range(0.0..1.0)
    }
}