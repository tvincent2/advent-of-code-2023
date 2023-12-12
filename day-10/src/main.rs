use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Cell {
    c: char,
    is_path: bool,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            c: value,
            is_path: false,
        }
    }
}

#[derive(Debug)]
struct PipeMap {
    map: Vec<Vec<Cell>>,
}

impl PipeMap {
    fn starting_point(&self) -> Option<(usize, usize)> {
        for (line_index, line) in self.iter().enumerate() {
            for (col_index, cell) in line.iter().enumerate() {
                if cell.c == 'S' {
                    return Some((line_index, col_index));
                }
            }
        }
        None
    }

    fn next_after_starting_point(&mut self) -> ((usize, usize), (usize, usize)) {
        let (s0, s1) = self.starting_point().expect("missing starting point");
        let max0 = self.len() - 1;
        let max1 = self[0].len() - 1;

        let mut points = vec![];
        let (mut up, mut down, mut left, mut right) = (false, false, false, false);
        // up
        if s0 > 0 && ['|', '7', 'F'].contains(&self[s0 - 1][s1].c) {
            points.push((s0 - 1, s1));
            up = true;
        }
        // down
        if s0 < max0 && ['|', 'L', 'J'].contains(&self[s0 + 1][s1].c) {
            points.push((s0 + 1, s1));
            down = true;
        }
        // left
        if s1 > 0 && ['-', 'L', 'F'].contains(&self[s0][s1 - 1].c) {
            points.push((s0, s1 - 1));
            left = true;
        }
        // right
        if s1 < max1 && ['-', 'J', '7'].contains(&self[s0][s1 + 1].c) {
            points.push((s0, s1 + 1));
            right = true;
        }
        assert_eq!(points.len(), 2);
        match (up, down, left, right) {
            (true, true, false, false) => self[s0][s1].c = '|',
            (true, false, true, false) => self[s0][s1].c = 'J',
            (true, false, false, true) => self[s0][s1].c = 'L',
            (false, true, true, false) => self[s0][s1].c = '7',
            (false, true, false, true) => self[s0][s1].c = 'F',
            (false, false, true, true) => self[s0][s1].c = '-',
            _ => unreachable!(),
        }
        ((points[0].0, points[0].1), (points[1].0, points[1].1))
    }

    fn next_point(&self, origin: (usize, usize), current: (usize, usize)) -> (usize, usize) {
        match self[current.0][current.1].c {
            '|' => {
                if origin.0 < current.0 {
                    (current.0 + 1, current.1)
                } else {
                    (current.0 - 1, current.1)
                }
            }
            '-' => {
                if origin.1 < current.1 {
                    (current.0, current.1 + 1)
                } else {
                    (current.0, current.1 - 1)
                }
            }
            'L' => {
                if origin.0 < current.0 {
                    (current.0, current.1 + 1)
                } else {
                    (current.0 - 1, current.1)
                }
            }
            'J' => {
                if origin.0 < current.0 {
                    (current.0, current.1 - 1)
                } else {
                    (current.0 - 1, current.1)
                }
            }
            '7' => {
                if origin.0 > current.0 {
                    (current.0, current.1 - 1)
                } else {
                    (current.0 + 1, current.1)
                }
            }
            'F' => {
                if origin.0 > current.0 {
                    (current.0, current.1 + 1)
                } else {
                    (current.0 + 1, current.1)
                }
            }
            _ => unreachable!(),
        }
    }

    fn mark_as_path(&mut self, (c0, c1): (usize, usize)) {
        self[c0][c1].is_path = true;
    }

    fn furthest_distance(&mut self) -> usize {
        let mut origin_a = self.starting_point().expect("missing starting point");
        let mut origin_b = origin_a.clone();
        self.mark_as_path(origin_a);
        let (mut current_a, mut current_b) = self.next_after_starting_point();
        self.mark_as_path(current_a);
        self.mark_as_path(current_b);
        let mut steps = 1;

        loop {
            if current_a == current_b {
                break;
            }
            let next_a = self.next_point(origin_a, current_a);
            let next_b = self.next_point(origin_b, current_b);
            origin_a = current_a;
            origin_b = current_b;
            current_a = next_a;
            current_b = next_b;
            self.mark_as_path(current_a);
            self.mark_as_path(current_b);
            steps += 1;
        }
        steps
    }

    fn count_inside_cells(&self) -> usize {
        let mut result = 0;
        self.iter().enumerate().for_each(|(lindex, line)| {
            let mut crossed = 0;
            let mut previous_corner_was_up = false;
            for (cindex, cell) in line.into_iter().enumerate() {
                if cell.is_path {
                    match cell.c {
                        '|' => crossed += 1,
                        'F' => previous_corner_was_up = false,
                        'L' => previous_corner_was_up = true,
                        '7' => {
                            if previous_corner_was_up {
                                crossed += 1
                            }
                        }
                        'J' => {
                            if !previous_corner_was_up {
                                crossed += 1
                            }
                        }
                        _ => {
                            // nothing
                        }
                    }
                } else {
                    if crossed % 2 == 1 {
                        println!("({}, {}) - {}", lindex, cindex, cell.c);
                        result += 1;
                    }
                }
            }
        });
        result
    }
}

impl From<&str> for PipeMap {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Self { map }
    }
}

impl Deref for PipeMap {
    type Target = Vec<Vec<Cell>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for PipeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

fn main() {
    let input = include_str!("../../input/day-10");
    let mut pipe_map = PipeMap::from(input);
    let distance = pipe_map.furthest_distance();
    println!("Distance: {}", distance);

    let inner_points = pipe_map.count_inside_cells();
    println!("Inner cells: {}", inner_points);
}

#[cfg(test)]
mod tests {
    use crate::PipeMap;

    #[test]
    fn starting_point() {
        let input = include_str!("../../input/day-10-test");
        let pipe_map = PipeMap::from(input);
        assert_eq!(pipe_map.starting_point(), Some((2, 0)));
    }
    #[test]
    fn next_after_starting_point() {
        let input = include_str!("../../input/day-10-test");
        let mut pipe_map = PipeMap::from(input);
        assert_eq!(pipe_map.next_after_starting_point(), ((3, 0), (2, 1)));
    }

    #[test]
    fn next_point() {
        let input = include_str!("../../input/day-10-test");
        let pipe_map = PipeMap::from(input);
        assert_eq!(pipe_map.next_point((2, 0), (3, 0)), (4, 0));
        assert_eq!(pipe_map.next_point((2, 0), (2, 1)), (1, 1));
    }

    #[test]
    fn furthest_distance() {
        let input = include_str!("../../input/day-10-test");
        let mut pipe_map = PipeMap::from(input);
        assert_eq!(pipe_map.furthest_distance(), 8);
    }

    #[test]
    fn count_inside_cells() {
        let input = include_str!("../../input/day-10-test2");
        let mut pipe_map = PipeMap::from(input);
        pipe_map.furthest_distance();
        assert_eq!(pipe_map.count_inside_cells(), 4);
    }

    #[test]
    fn count_inside_cells2() {
        let input = include_str!("../../input/day-10-test3");
        let mut pipe_map = PipeMap::from(input);
        pipe_map.furthest_distance();
        assert_eq!(pipe_map.count_inside_cells(), 8);
    }

    #[test]
    fn count_inside_cells3() {
        let input = include_str!("../../input/day-10-test4");
        let mut pipe_map = PipeMap::from(input);
        pipe_map.furthest_distance();
        assert_eq!(pipe_map.count_inside_cells(), 10);
    }
}
