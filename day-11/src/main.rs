#[derive(Debug, PartialEq)]
struct Galaxy {
    coords: (usize, usize),
}

impl Galaxy {
    fn distance_to(&self, other: &Galaxy) -> usize {
        self.coords.0.abs_diff(other.coords.0) + self.coords.1.abs_diff(other.coords.1)
    }
}

#[derive(Debug, PartialEq)]
struct Universe {
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn sum_of_distances(&self) -> usize {
        let mut sum = 0;
        for index in 0..self.galaxies.len() {
            let current_galaxy = &self.galaxies[index];
            for other_galaxy in &self.galaxies[index + 1..self.galaxies.len()] {
                sum += current_galaxy.distance_to(other_galaxy);
            }
        }
        sum
    }
}

impl From<&str> for Universe {
    fn from(input: &str) -> Self {
        let lines_without_galaxies: Vec<bool> = input
            .lines()
            .map(|line| line.chars().all(|c| c == '.'))
            .collect();
        let mut cols_have_no_galaxies: Vec<bool> =
            vec![true; input.lines().next().expect("no line in file").len()];
        input.lines().for_each(|line| {
            line.char_indices().for_each(|(index, c)| {
                if c == '#' {
                    cols_have_no_galaxies[index] = false
                }
            })
        });

        let mut galaxies = vec![];
        let mut expanded_line_index = 0;
        for (line_index, line) in input.lines().enumerate() {
            if lines_without_galaxies[line_index] {
                expanded_line_index += 999_999;
            } else {
                let mut expanded_col_index = 0;
                for (col_index, col) in line.char_indices() {
                    if cols_have_no_galaxies[col_index] {
                        expanded_col_index += 999_999;
                    } else {
                        if col == '#' {
                            galaxies.push(Galaxy {
                                coords: (expanded_line_index, expanded_col_index),
                            });
                        }
                    }
                    expanded_col_index += 1;
                }
            }
            expanded_line_index += 1;
        }
        Universe { galaxies }
    }
}

fn main() {
    let input = include_str!("../../input/day-11");
    let universe = Universe::from(input);
    let sum = universe.sum_of_distances();
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::{Galaxy, Universe};

    #[test]
    fn expanding_universe() {
        let input = include_str!("../../input/day-11-test");
        let universe = Universe::from(input);
        assert_eq!(
            universe,
            Universe {
                galaxies: vec![
                    Galaxy { coords: (0, 4) },
                    Galaxy { coords: (1, 9) },
                    Galaxy { coords: (2, 0) },
                    Galaxy { coords: (5, 8) },
                    Galaxy { coords: (6, 1) },
                    Galaxy { coords: (7, 12) },
                    Galaxy { coords: (10, 9) },
                    Galaxy { coords: (11, 0) },
                    Galaxy { coords: (11, 5) }
                ]
            }
        );
    }

    #[test]
    fn sum_of_distances() {
        let input = include_str!("../../input/day-11-test");
        let universe = Universe::from(input);
        assert_eq!(universe.sum_of_distances(), 374);
    }
}
