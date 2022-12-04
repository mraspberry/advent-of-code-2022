use crate::types::elf::Elf;
use derive_new::new;

#[derive(Debug, new)]
pub struct Expedition {
    elfs: Vec<Elf>,
}

impl Expedition {
    pub fn most_calories(&self) -> usize {
        self.elfs.iter().map(|x| x.count_calories()).max().unwrap()
    }

    pub fn add_elf(&mut self, new_elf: Elf) {
        self.elfs.push(new_elf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expedition_most_calories() {
        let expedition = Expedition::new(vec![Elf::new(vec![2000]), Elf::new(vec![1000])]);
        assert_eq!(2000usize, expedition.most_calories());
    }
}
