use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct Elf {
    calories: Vec<usize>,
}

impl Elf {
    pub fn count_calories(&self) -> usize {
        self.calories.iter().sum()
    }

    pub fn add_meal(&mut self, meal: usize) {
        self.calories.push(meal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_max_calories() {
        let calories: Vec<usize> = vec![1000, 2000];
        let elf = Elf::new(calories);
        assert_eq!(3000, elf.count_calories());
    }
}
