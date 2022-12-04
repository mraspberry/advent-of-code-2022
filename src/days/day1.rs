use crate::types::elf::Elf;
use crate::types::expedition::Expedition;

pub fn puzzle1() -> usize {
    let expedition = build_expedition();
    expedition.most_calories()
}

fn build_expedition() -> Expedition {
    let mut expedition = Expedition::new(vec![]);
    let mut current = Elf::new(vec![]);
    let data = include_str!("../../data/day1_puzzle1.txt");
    for line in data.lines() {
        match line.parse::<usize>() {
            Ok(value) => current.add_meal(value),
            Err(_) => {
                expedition.add_elf(current.clone());
                current = Elf::new(vec![]);
            }
        }
    }
    expedition
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1_answer() {
        let ans: usize = 70764;
        assert_eq!(ans, puzzle1());
    }
}
