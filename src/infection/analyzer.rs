pub mod dumbass; pub use dumbass::Dumbass;
pub mod basic; pub use basic::Basic;
pub mod surrounder; pub use surrounder::Surrounder;

use crate::infection::Field;
pub trait Analyzer {
    fn heuristic(&self, player_id: u8, field: Field) -> f32;
}
