mod days;
pub mod types;
use days::*;

pub fn get_days() -> &'static [(fn() -> usize, &'static str)] {
    &[(day1::puzzle1, "Day1|Puzzle1")]
}
