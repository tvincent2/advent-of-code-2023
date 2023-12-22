use std::collections::HashMap;

struct City {
    blocks: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    U,
    D,
    L,
    R,
}

enum Part {
    One,
    Two,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Crucible {
    position: (usize, usize),
    direction: Direction,
    line_len: usize,
}

impl Crucible {
    fn can_go_further(&self, part: &Part) -> bool {
        match part {
            Part::One => self.line_len < 3,
            Part::Two => self.line_len < 10,
        }
    }

    fn min_steps(&self, part: &Part) -> usize {
        match part {
            Part::One => 1,
            Part::Two => 4,
        }
    }

    fn went_far_enough(&self, part: &Part) -> bool {
        self.line_len >= self.min_steps(part)
    }
}

impl City {
    // the purpose of this function is to provide a feasible solution that will act as an upper bound value during the search
    fn diagonal_path(&self) -> usize {
        let width = self.blocks[0].len();
        let mut line_index = 0;
        let mut sum = 0;
        for col_index in 1..width {
            sum += self.blocks[line_index][col_index] + self.blocks[line_index + 1][col_index];
            line_index += 1;
        }
        sum
    }

    // same as diagonal_path, but for part 2
    fn diagonal_path_part_2(&self) -> usize {
        let width = self.blocks[0].len();
        let width_to_do_by_ten = match width % 10 {
            1 | 2 | 3 => width - 1, // we need to stop one step earlier to make 7 + 4, 8 + 4 or 9 + 4 for the last two steps
            _ => width,
        };
        let height = self.blocks.len();
        let mut index = 0;
        let mut sum = 0;
        for col_index in 1..(width_to_do_by_ten / 10) {
            let col_range = (col_index - 1) * 10 + 1..col_index * 10;
            let line_range = index..index + 11;
            sum += self.blocks[index][col_range].iter().sum::<usize>()
                + self.blocks[line_range]
                    .iter()
                    .map(|line| line[col_index])
                    .sum::<usize>();
            index += 10;
        }
        match width % 10 {
            0 => {
                // we're done
            }
            x if x < 4 => {
                let step = if x == 1 {
                    7 + 1
                } else if x == 2 {
                    8 + 1
                } else {
                    9 + 1
                };
                sum += self.blocks[index][index + 1..index + step]
                    .iter()
                    .sum::<usize>()
                    + self.blocks[index + 1..index + step]
                        .iter()
                        .map(|line| line[index + step - 1])
                        .sum::<usize>();
                index += step;
                sum += self.blocks[index][index + 1..width].iter().sum::<usize>()
                    + self.blocks[index + 1..height]
                        .iter()
                        .map(|line| line[width - 1])
                        .sum::<usize>();
            }
            x if x < 10 => {
                sum += self.blocks[index][index + 1..width].iter().sum::<usize>()
                    + self.blocks[index + 1..height]
                        .iter()
                        .map(|line| line[width - 1])
                        .sum::<usize>();
            }
            _ => unreachable!(),
        }
        sum
    }

    fn dfs(&self, part: Part) -> usize {
        let mut cache = HashMap::new();
        let mut best_known_heat = match part {
            Part::One => self.diagonal_path(),
            Part::Two => self.diagonal_path_part_2(),
        };
        self.dfs_rec(
            Crucible {
                position: (1, 0),
                direction: Direction::R,
                line_len: 1,
            },
            self.blocks[0][1],
            &mut best_known_heat,
            &mut cache,
            &part,
        );
        self.dfs_rec(
            Crucible {
                position: (0, 1),
                direction: Direction::D,
                line_len: 1,
            },
            self.blocks[1][0],
            &mut best_known_heat,
            &mut cache,
            &part,
        );
        best_known_heat
    }

    fn dfs_rec(
        &self,
        crucible: Crucible,
        heat: usize,
        best_known_heat: &mut usize,
        cache: &mut HashMap<Crucible, usize>,
        part: &Part,
    ) {
        let width = self.blocks[0].len();
        let height = self.blocks.len();

        if heat > *best_known_heat {
            return;
        }

        if heat < *cache.get(&crucible).unwrap_or(&usize::MAX) {
            cache.insert(crucible.clone(), heat);
        } else {
            return;
        }

        let (x, y) = crucible.position;

        if (x, y) == (width - 1, height - 1) {
            if heat < *best_known_heat {
                *best_known_heat = heat;
            }
            return;
        }

        // up
        if y > 0 && crucible.direction != Direction::D {
            let next_heat = heat + self.blocks[y - 1][x];

            if crucible.direction != Direction::U {
                if crucible.went_far_enough(part) && y >= crucible.min_steps(part) {
                    self.dfs_rec(
                        Crucible {
                            position: (x, y - 1),
                            direction: Direction::U,
                            line_len: 1,
                        },
                        next_heat,
                        best_known_heat,
                        cache,
                        part,
                    );
                }
            } else if crucible.can_go_further(part) {
                self.dfs_rec(
                    Crucible {
                        position: (x, y - 1),
                        direction: Direction::U,
                        line_len: crucible.line_len + 1,
                    },
                    next_heat,
                    best_known_heat,
                    cache,
                    part,
                );
            }
        }
        // down
        if y < height - 1 && crucible.direction != Direction::U {
            let next_heat = heat + self.blocks[y + 1][x];
            if crucible.direction != Direction::D {
                if crucible.went_far_enough(part) && y < height - crucible.min_steps(part) {
                    self.dfs_rec(
                        Crucible {
                            position: (x, y + 1),
                            direction: Direction::D,
                            line_len: 1,
                        },
                        next_heat,
                        best_known_heat,
                        cache,
                        part,
                    );
                }
            } else if crucible.can_go_further(part) {
                self.dfs_rec(
                    Crucible {
                        position: (x, y + 1),
                        direction: Direction::D,
                        line_len: crucible.line_len + 1,
                    },
                    next_heat,
                    best_known_heat,
                    cache,
                    part,
                );
            }
        }
        // left
        if x > 0 && crucible.direction != Direction::R {
            let next_heat = heat + self.blocks[y][x - 1];
            if crucible.direction != Direction::L {
                if crucible.went_far_enough(part) && x >= crucible.min_steps(part) {
                    self.dfs_rec(
                        Crucible {
                            position: (x - 1, y),
                            direction: Direction::L,
                            line_len: 1,
                        },
                        next_heat,
                        best_known_heat,
                        cache,
                        part,
                    );
                }
            } else if crucible.can_go_further(part) {
                self.dfs_rec(
                    Crucible {
                        position: (x - 1, y),
                        direction: Direction::L,
                        line_len: crucible.line_len + 1,
                    },
                    next_heat,
                    best_known_heat,
                    cache,
                    part,
                );
            }
        }
        // right
        if x < width - 1 && crucible.direction != Direction::L {
            let next_heat = heat + self.blocks[y][x + 1];
            if crucible.direction != Direction::R {
                if crucible.went_far_enough(part) && x < width - crucible.min_steps(part) {
                    self.dfs_rec(
                        Crucible {
                            position: (x + 1, y),
                            direction: Direction::R,
                            line_len: 1,
                        },
                        next_heat,
                        best_known_heat,
                        cache,
                        part,
                    );
                }
            } else if crucible.can_go_further(part) {
                self.dfs_rec(
                    Crucible {
                        position: (x + 1, y),
                        direction: Direction::R,
                        line_len: crucible.line_len + 1,
                    },
                    next_heat,
                    best_known_heat,
                    cache,
                    part,
                );
            }
        }
    }
}

impl From<&str> for City {
    fn from(input: &str) -> Self {
        let blocks = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().expect("unparsable number"))
                    .collect()
            })
            .collect();
        Self { blocks }
    }
}

fn main() {
    let input = include_str!("../../input/day-17");
    let city = City::from(input);
    let lowest_heat_loss = city.dfs(Part::One);
    println!("Lowest heat loss: {}", lowest_heat_loss);

    let lowest_heat_loss_with_ultra_crucibles = city.dfs(Part::Two);
    println!(
        "Lowest heat loss with ultra crucibles: {}",
        lowest_heat_loss_with_ultra_crucibles
    );
}

#[cfg(test)]
mod tests {
    use crate::City;

    #[test]
    fn lowest_heat_loss() {
        let input = include_str!("../../input/day-17-test");
        let city = City::from(input);
        assert_eq!(city.dfs(crate::Part::One), 102);
    }

    #[test]
    fn diag() {
        let input = include_str!("../../input/day-17-test");
        let city = City::from(input);
        assert!(city.diagonal_path() >= 102);
    }

    #[test]
    fn diag2() {
        let input = include_str!("../../input/day-17-test");
        let city = City::from(input);
        assert!(city.diagonal_path_part_2() >= 94);
    }

    #[test]
    fn lowest_heat_loss2() {
        let input = include_str!("../../input/day-17-test");
        let city = City::from(input);
        assert_eq!(city.dfs(crate::Part::Two), 94);
    }
}
