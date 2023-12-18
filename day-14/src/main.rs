use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Platform {
    lines: Vec<String>,
}

impl Platform {
    fn tilt_west(&self) -> Self {
        let lines = self
            .lines
            .iter()
            .map(|line| {
                let mut string = String::with_capacity(line.len());
                for (index, c) in line.char_indices() {
                    match c {
                        'O' => string.push('O'),
                        '#' => {
                            (string.len()..index).for_each(|_| string.push('.'));
                            string.push('#')
                        }
                        _ => {
                            // nothing to do
                        }
                    }
                }
                (string.len()..line.len()).for_each(|_| string.push('.'));
                string
            })
            .collect();
        Self { lines }
    }

    fn tilt_east(&self) -> Self {
        let lines = self
            .lines
            .iter()
            .map(|line| {
                let mut string = String::with_capacity(line.len());
                for (index, c) in line.chars().rev().enumerate() {
                    match c {
                        'O' => string.push('O'),
                        '#' => {
                            (string.len()..index).for_each(|_| string.push('.'));
                            string.push('#')
                        }
                        _ => {
                            // nothing to do
                        }
                    }
                }
                (string.len()..line.len()).for_each(|_| string.push('.'));
                string.chars().rev().collect()
            })
            .collect();
        Self { lines }
    }

    fn tilt_north(&self) -> Self {
        let width = self.lines[0].len();
        let height = self.lines.len();
        let mut lines = vec![String::with_capacity(width); height];
        let mut first_available_spot = vec![0; width];
        for c_index in 0..width {
            for (l_index, line) in self.lines.iter().enumerate() {
                match line.chars().nth(c_index) {
                    Some('O') => {
                        lines[first_available_spot[c_index]].push('O');
                        first_available_spot[c_index] += 1
                    }
                    Some('#') => {
                        lines[l_index].push('#');
                        first_available_spot[c_index] = l_index + 1;
                    }
                    _ => {
                        // nothing to do
                    }
                }
            }
            for index in 0..lines.len() {
                if lines[index].len() < c_index + 1 {
                    lines[index].push('.');
                }
            }
        }
        Self { lines }
    }

    fn tilt_south(&self) -> Self {
        let width = self.lines[0].len();
        let height = self.lines.len();
        let mut lines = vec![String::with_capacity(width); height];
        let mut first_available_spot = vec![height - 1; width];
        for c_index in 0..width {
            for (l_index, line) in self.lines.iter().enumerate().rev() {
                match line.chars().nth(c_index) {
                    Some('O') => {
                        lines[first_available_spot[c_index]].push('O');
                        if first_available_spot[c_index] > 0 {
                            first_available_spot[c_index] -= 1;
                        }
                    }
                    Some('#') => {
                        lines[l_index].push('#');
                        if l_index > 0 {
                            first_available_spot[c_index] = l_index - 1;
                        }
                    }
                    _ => {
                        // nothing to do
                    }
                }
            }
            for index in 0..lines.len() {
                if lines[index].len() < c_index + 1 {
                    lines[index].push('.');
                }
            }
        }
        Self { lines }
    }

    fn cycle(&self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    fn count_north_load(&self) -> usize {
        let height = self.lines.len();
        self.lines
            .iter()
            .enumerate()
            .map(|(index, line)| line.chars().filter(|&c| c == 'O').count() * (height - index))
            .sum()
    }

    fn find_repetitions(&self) -> (HashMap<Platform, usize>, usize, usize) {
        let mut iteration = 0;

        let mut repetitions = HashMap::new();
        repetitions.insert(self.clone(), iteration);
        let mut current_platform = self.clone();
        let mut id = 0;
        loop {
            current_platform = current_platform.cycle();
            iteration += 1;
            if let Some(&index) = repetitions.get(&current_platform) {
                id = index;
                break;
            } else {
                repetitions.insert(current_platform.clone(), iteration);
            }
        }
        (repetitions, id, iteration)
    }

    fn load_after_one_billion_cycles(&self) -> usize {
        let (repetition_hashmap, index_in_hashmap, iteration) = self.find_repetitions();
        let index =
            (1_000_000_000 - index_in_hashmap) % (iteration - index_in_hashmap) + index_in_hashmap;
        repetition_hashmap
            .iter()
            .find(|(_, &i)| index == i)
            .map(|(platform, _)| platform.count_north_load())
            .unwrap()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lines.iter().for_each(|line| {
            writeln!(f, "{line}").expect("line not printable");
        });
        writeln!(f, "\n")
    }
}

impl From<&str> for Platform {
    fn from(input: &str) -> Self {
        let lines = input.lines().map(|line| line.to_string()).collect();
        Platform { lines }
    }
}

fn main() {
    let input = include_str!("../../input/day-14");
    let platform = Platform::from(input);
    let tilted_north_platform = platform.tilt_north();
    let total_load = tilted_north_platform.count_north_load();
    println!("Total load: {}", total_load);

    let total_load_after_one_billion_cycles = platform.load_after_one_billion_cycles();
    println!(
        "Total load after one billion cycles: {}",
        total_load_after_one_billion_cycles
    );
}

#[cfg(test)]
mod tests {
    use crate::Platform;

    #[test]
    fn sum_load() {
        let input = include_str!("../../input/day-14-test");
        let platform = Platform::from(input);
        let tilted_north_platform = platform.tilt_north();
        assert_eq!(tilted_north_platform.count_north_load(), 136);
    }

    #[test]
    fn three_cycles() {
        let input = include_str!("../../input/day-14-test");
        let platform = Platform::from(input);
        let platform2 = platform.cycle();
        println!("{platform2}");
        let platform3 = platform2.cycle();
        println!("{platform3}");
        let platform4 = platform3.cycle();
        println!("{platform4}");
        assert!(true);
    }

    #[test]
    fn one_billion_cycles() {
        let input = include_str!("../../input/day-14-test");
        let platform = Platform::from(input);
        assert_eq!(platform.load_after_one_billion_cycles(), 64);
    }
}
