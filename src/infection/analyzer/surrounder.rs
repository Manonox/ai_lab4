use crate::bot::Game;
use crate::infection::analyzer::Analyzer;
use crate::infection::field::*;


// fn manhattan_distance(p1: [f32; 2], p2: [f32; 2]) -> f32 {
//     (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
// }

fn count_cloning_moves(field: Field) -> usize {
    field.valid_moves().into_iter().filter(|m| m.is_cloning()).count()
}

pub struct Surrounder;
impl Analyzer for Surrounder {
    fn heuristic(&self, player_id: u8, field: Field) -> f32 {
        use crate::infection::analyzer::Basic;
        let base = Basic.heuristic(player_id, field);
        base * base * 0.1 * base.signum() + (count_cloning_moves(field) as f32) * 0.25 * if field.current_player_id == player_id {1.0} else {-1.0}
    }
}