use rand::Rng;


pub trait Game<Move> {
    fn valid_moves(&self) -> Vec<Move>;
    fn make_move(&mut self, m: Move) -> Result<Option<u8>, ()>;
}


use crate::infection::field::*;
use crate::infection::analyzer::Analyzer;

pub struct Bot {
    max_depth: u32,
    analyzer: Box<dyn Analyzer>,
}

impl Bot {
    pub fn new(max_depth: u32, analyzer: Box<dyn Analyzer>) -> Bot {
        Bot { max_depth, analyzer }
    }

    fn heuristic(&self, player_id: u8, field: Field) -> f32 {
        self.analyzer.heuristic(player_id, field) + rand::thread_rng().gen_range(-0.01..0.01)
    }

    pub fn decide(&self, field: Field) -> Option<Move> {
        let player_id = field.current_player_id;
        

        let valid_moves = field.valid_moves();
        if valid_moves.is_empty() { return None }

        let mut value= None;
        let mut good_m = None;

        for m in valid_moves {
            let mut nextfield = field.clone();
            let newvalue;
            if let Some(winner) = nextfield.make_move(m).unwrap() {
                newvalue = if winner == player_id { f32::INFINITY } else { f32::NEG_INFINITY };
            } else {
                newvalue = self.minimax(nextfield, player_id, 1);
            }

            println!("{m} Score: {newvalue}");
            if value.is_none() || newvalue > value.unwrap() {
                value = Some(newvalue);
                good_m = Some(m);
            }
        }
        good_m
    }

    fn minimax(&self, field: Field, player_id: u8, depth: u32) -> f32 {
        if depth >= self.max_depth { return self.heuristic(player_id, field) }
        let maximize = player_id == field.current_player_id;
        if maximize {
            let mut value = f32::NEG_INFINITY;
            field.valid_moves().into_iter().for_each(|m| {
                let mut nextfield = field.clone();
                if let Some(winner) = nextfield.make_move(m).unwrap() {
                    value = if winner == player_id { f32::INFINITY } else { f32::NEG_INFINITY };
                    return;
                }

                let nextvalue = self.minimax(nextfield, player_id, depth + 1);
                value = value.max(nextvalue);
            });
            return value;
        }
        else {
            let mut value = f32::INFINITY;
            field.valid_moves().into_iter().for_each(|m| {
                let mut nextfield = field.clone();
                if let Some(winner) = nextfield.make_move(m).unwrap() {
                    value = if winner == player_id { f32::INFINITY } else { f32::NEG_INFINITY };
                    return;
                }
                let nextvalue = self.minimax(nextfield, player_id, depth + 1);
                value = value.min(nextvalue);
            });
            return value;
        }
    }
}
