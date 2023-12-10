use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
struct Number {
    value: usize,
    is_part: bool,
    interval: (usize, usize),
}

impl Number {
    fn has_matching_interval(&self, coord: usize) -> bool {
        // specific case when the interval starts at 0, since we can't use -1 with usize
        if self.interval.0 == 0 {
            coord <= self.interval.1 + 1
        } else {
            coord >= self.interval.0 - 1 && coord <= self.interval.1 + 1
        }
    }
}

#[derive(Debug, PartialEq)]
struct Numbers {
    numbers: Vec<Number>,
}

impl Deref for Numbers {
    type Target = Vec<Number>;

    fn deref(&self) -> &Self::Target {
        &self.numbers
    }
}

impl DerefMut for Numbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.numbers
    }
}

impl From<&str> for Numbers {
    fn from(input: &str) -> Self {
        let mut numbers = vec![];
        let mut interv_end: Option<usize> = None;
        let mut multiplier = 1;
        let mut current_number = 0;
        input.char_indices().rev().for_each(|(index, char)| {
            match char.to_string().parse::<usize>() {
                Ok(number) => {
                    current_number += multiplier * number;
                    multiplier *= 10;
                    if interv_end.is_none() {
                        interv_end = Some(index);
                    }
                }
                Err(_) => {
                    if let Some(int_end) = interv_end {
                        numbers.push(Number {
                            value: current_number,
                            is_part: false,
                            interval: (index + 1, int_end),
                        });
                    }
                    interv_end = None;
                    multiplier = 1;
                    current_number = 0;
                }
            }
        });
        if let Some(int_end) = interv_end {
            numbers.push(Number {
                value: current_number,
                is_part: false,
                interval: (0, int_end),
            });
        }
        numbers.reverse();
        Self { numbers }
    }
}

#[derive(Debug, PartialEq)]
struct Symbol {
    coords: (usize, usize),
    is_gear: bool,
    gear_ratio: usize,
}

#[derive(Debug, PartialEq)]
struct EngineParts {
    parts: Vec<Numbers>,
    symbols: Vec<Symbol>,
}

impl From<&str> for EngineParts {
    fn from(value: &str) -> Self {
        let parts = value.lines().map(|line| line.into()).collect();
        let symbols = value
            .lines()
            .enumerate()
            .flat_map(|(line_index, line)| {
                line.char_indices()
                    .filter(|(_, c)| is_symbol(c))
                    .map(move |(col_index, c)| Symbol {
                        coords: (line_index, col_index),
                        is_gear: c == '*',
                        gear_ratio: 0,
                    })
            })
            .collect();
        let mut result = Self { parts, symbols };
        result.mark_parts();
        result
    }
}

impl Deref for EngineParts {
    type Target = Vec<Numbers>;

    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}

impl DerefMut for EngineParts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parts
    }
}

impl EngineParts {
    fn mark_parts(&mut self) {
        let number_of_lines = self.parts.len();
        self.symbols.iter_mut().for_each(|symbol| {
            let (line_coord, col_coord) = symbol.coords;
            let min_coord = if line_coord == 0 {
                line_coord.clone()
            } else {
                line_coord - 1
            };
            let max_coord = if line_coord == number_of_lines - 1 {
                line_coord.clone()
            } else {
                line_coord + 1
            };
            let mut ratio = 1;
            let mut number_of_parts = 0;
            for line_index in min_coord..=max_coord {
                self.parts[line_index]
                    .iter_mut()
                    .filter(|part| part.has_matching_interval(col_coord))
                    .for_each(|part| {
                        part.is_part = true;
                        number_of_parts += 1;
                        ratio *= part.value;
                    })
            }
            if symbol.is_gear && number_of_parts == 2 {
                symbol.gear_ratio = ratio;
            }
        });
    }

    fn sum_of_parts(&self) -> usize {
        self.parts
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|part| part.is_part)
                    .map(|part| part.value)
                    .sum::<usize>()
            })
            .sum()
    }

    fn sum_of_gear_ratios(&self) -> usize {
        self.symbols.iter().map(|symbol| symbol.gear_ratio).sum()
    }
}

fn is_symbol(c: &char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => false,
        _ => true,
    }
}

fn main() {
    let input = include_str!("../../input/day-03");
    let engine = EngineParts::from(input);

    let sum_of_parts = engine.sum_of_parts();
    println!("Part 1 - Sum: {}", sum_of_parts);

    let sum_of_gear_ratios = engine.sum_of_gear_ratios();
    println!("Part 2 - Sum: {}", sum_of_gear_ratios);
}

#[cfg(test)]
mod tests {
    use crate::EngineParts;
    use crate::Number;
    use crate::Numbers;
    use crate::Symbol;

    #[test]
    fn simple_line_number_parse() {
        let numbers = Numbers::from("467..114..");
        assert_eq!(
            numbers,
            Numbers {
                numbers: vec![
                    Number {
                        value: 467,
                        is_part: false,
                        interval: (0, 2)
                    },
                    Number {
                        value: 114,
                        is_part: false,
                        interval: (5, 7)
                    }
                ]
            }
        );
    }

    #[test]
    fn file_number_parse() {
        let input = include_str!("../../input/day-03-test");
        let numbers = EngineParts::from(input);
        let expected_engine_numbers = EngineParts {
            parts: vec![
                Numbers {
                    numbers: vec![
                        Number {
                            value: 467,
                            is_part: true,
                            interval: (0, 2),
                        },
                        Number {
                            value: 114,
                            is_part: false,
                            interval: (5, 7),
                        },
                    ],
                },
                Numbers { numbers: vec![] },
                Numbers {
                    numbers: vec![
                        Number {
                            value: 35,
                            is_part: true,
                            interval: (2, 3),
                        },
                        Number {
                            value: 633,
                            is_part: true,
                            interval: (6, 8),
                        },
                    ],
                },
                Numbers { numbers: vec![] },
                Numbers {
                    numbers: vec![Number {
                        value: 617,
                        is_part: true,
                        interval: (0, 2),
                    }],
                },
                Numbers {
                    numbers: vec![Number {
                        value: 58,
                        is_part: false,
                        interval: (7, 8),
                    }],
                },
                Numbers {
                    numbers: vec![Number {
                        value: 592,
                        is_part: true,
                        interval: (2, 4),
                    }],
                },
                Numbers {
                    numbers: vec![Number {
                        value: 755,
                        is_part: true,
                        interval: (6, 8),
                    }],
                },
                Numbers { numbers: vec![] },
                Numbers {
                    numbers: vec![
                        Number {
                            value: 664,
                            is_part: true,
                            interval: (1, 3),
                        },
                        Number {
                            value: 598,
                            is_part: true,
                            interval: (5, 7),
                        },
                    ],
                },
            ],
            symbols: vec![
                Symbol {
                    coords: (1, 3),
                    is_gear: true,
                    gear_ratio: 16345,
                },
                Symbol {
                    coords: (3, 6),
                    is_gear: false,
                    gear_ratio: 0,
                },
                Symbol {
                    coords: (4, 3),
                    is_gear: true,
                    gear_ratio: 0,
                },
                Symbol {
                    coords: (5, 5),
                    is_gear: false,
                    gear_ratio: 0,
                },
                Symbol {
                    coords: (8, 3),
                    is_gear: false,
                    gear_ratio: 0,
                },
                Symbol {
                    coords: (8, 5),
                    is_gear: true,
                    gear_ratio: 451490,
                },
            ],
        };
        assert_eq!(numbers, expected_engine_numbers);
    }

    #[test]
    fn sum_of_parts() {
        let input = include_str!("../../input/day-03-test");
        let engine = EngineParts::from(input);
        let sum = engine.sum_of_parts();
        assert_eq!(sum, 4361);
    }

    #[test]
    fn sum_of_gear_ratios() {
        let input = include_str!("../../input/day-03-test");
        let engine = EngineParts::from(input);
        let sum = engine.sum_of_gear_ratios();
        assert_eq!(sum, 467835);
    }
}
