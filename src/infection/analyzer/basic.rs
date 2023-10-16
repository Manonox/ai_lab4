use crate::infection::analyzer::Analyzer;
use crate::infection::field::*;


fn manhattanish_distance(p1: [f32; 2], p2: [f32; 2]) -> f32 {
    (p1[0] - p2[0]).abs().max((p1[1] - p2[1]).abs())
}

fn calculate_cell_cost(p: [i8; 2]) -> f32 {
    1.0 + (3.5 - manhattanish_distance([p[0] as f32, p[1] as f32], [2.5, 2.5])) / 3.5
}

pub struct Basic;
impl Analyzer for Basic {
    fn heuristic(&self, player_id: u8, field: Field) -> f32 {
        field.into_iter().filter_map(|p| {
            let cell = field.get(p).unwrap();
            if cell == 0 { return None }
            let cell_cost = calculate_cell_cost(p);
            Some(if player_id == cell { cell_cost } else { -cell_cost })
        }).sum()
    }
}