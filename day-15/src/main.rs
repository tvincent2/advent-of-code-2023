struct Hasher {
    pub digest: u32,
}

impl Hasher {
    fn new() -> Self {
        Self { digest: 0 }
    }

    fn reset(&mut self) {
        self.digest = 0;
    }

    fn add_ascii_char(&mut self, c: char) {
        let code = c as u32;
        let new_digest = ((self.digest + code) * 17) % 256;
        self.digest = new_digest;
    }

    fn hash_str(&mut self, str: &str) -> u32 {
        str.split(",")
            .map(|s| {
                self.reset();
                s.chars()
                    .filter(|&c| c != '\n')
                    .for_each(|c| self.add_ascii_char(c));
                self.digest
            })
            .sum()
    }
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal: usize,
}

#[derive(Clone, Debug)]
struct Box {
    slots: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Box { slots: vec![] }
    }

    fn remove_lens(&mut self, lens_label: &str) {
        if let Some((index, _)) = self
            .slots
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.label == lens_label)
        {
            self.slots.remove(index);
        }
    }

    fn add_or_replace_lens(&mut self, lens_to_add: Lens) {
        if let Some((index, _)) = self
            .slots
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.label == lens_to_add.label)
        {
            self.slots[index] = lens_to_add;
        } else {
            self.slots.push(lens_to_add)
        }
    }

    fn count_focusing_power(&self) -> usize {
        self.slots
            .iter()
            .enumerate()
            .map(|(index, lens)| (index + 1) * lens.focal)
            .sum()
    }
}

#[derive(Debug)]
struct Boxes {
    boxes: Vec<Box>,
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: vec![Box::new(); 256],
        }
    }

    fn process_instruction(&mut self, instruction: &str) {
        let mut hasher = Hasher::new();
        let label_length = if instruction.ends_with("-") {
            instruction.len() - 1
        } else {
            instruction.len() - 2
        };
        let label = &instruction[..label_length];
        let box_index = hasher.hash_str(label) as usize;

        if instruction.ends_with("-") {
            self.boxes[box_index].remove_lens(label);
        } else {
            let focal = instruction
                .chars()
                .last()
                .expect("empty instruction")
                .to_digit(10)
                .expect("not a digit");
            self.boxes[box_index].add_or_replace_lens(Lens {
                label: label.to_string(),
                focal: focal as usize,
            });
        }
    }

    fn process_instructions(&mut self, input: &str) {
        input
            .split(",")
            .for_each(|instruction| self.process_instruction(instruction));
    }

    fn count_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(index, b)| (index + 1) * b.count_focusing_power())
            .sum()
    }
}

fn main() {
    let input = include_str!("../../input/day-15");
    let mut hasher = Hasher::new();
    let hash = hasher.hash_str(input);
    println!("Global hash: {}", hash);

    let mut boxes = Boxes::new();
    boxes.process_instructions(input);
    let focal_power = boxes.count_focusing_power();
    println!("Total power: {}", focal_power);
}

#[cfg(test)]
mod tests {
    use crate::{Boxes, Hasher};

    #[test]
    fn hash() {
        let input = "HASH";
        let mut hasher = Hasher::new();
        input.chars().for_each(|c| hasher.add_ascii_char(c));
        assert_eq!(hasher.digest, 52);
    }

    #[test]
    fn hash_test_input() {
        let input = include_str!("../../input/day-15-test");
        let mut hasher = Hasher::new();
        assert_eq!(hasher.hash_str(input), 1320);
    }

    #[test]
    fn focusing_power() {
        let input = include_str!("../../input/day-15-test");
        let mut boxes = Boxes::new();
        boxes.process_instructions(input);
        assert_eq!(boxes.count_focusing_power(), 145);
    }
}
