enum TileKind {
    Empty,
    VSplitter,
    HSplitter,
    LMirror,
    RMirror,
}

struct Tile {
    kind: TileKind,
    energized: bool,
}

impl Tile {
    fn is_energized(&self) -> bool {
        self.energized
    }

    fn energize(&mut self) {
        self.energized = true;
    }

    fn de_energize(&mut self) {
        self.energized = false;
    }
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        let kind = match input {
            '.' => TileKind::Empty,
            '-' => TileKind::HSplitter,
            '|' => TileKind::VSplitter,
            '/' => TileKind::RMirror,
            '\\' => TileKind::LMirror,
            _ => unreachable!(),
        };
        Self {
            kind,
            energized: false,
        }
    }
}

enum Direction {
    U,
    D,
    L,
    R,
}

struct LightBeam {
    direction: Direction,
    x: usize,
    y: usize,
}

impl LightBeam {
    fn left(&self) -> Self {
        Self {
            direction: Direction::L,
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Self {
            direction: Direction::R,
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(&self) -> Self {
        Self {
            direction: Direction::U,
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Self {
        Self {
            direction: Direction::D,
            x: self.x,
            y: self.y + 1,
        }
    }

    fn next(&self, tile: &Tile, max_x: usize, max_y: usize) -> Option<Vec<LightBeam>> {
        match (&self.direction, &tile.kind) {
            (Direction::U, TileKind::Empty) | (Direction::U, TileKind::VSplitter) => {
                if self.y == 0 {
                    None
                } else {
                    Some(vec![self.up()])
                }
            }
            (Direction::U, TileKind::HSplitter) | (Direction::D, TileKind::HSplitter) => {
                if tile.is_energized() {
                    None
                } else {
                    let mut result = vec![];
                    if self.x > 0 {
                        result.push(self.left());
                    }
                    if self.x < max_x {
                        result.push(self.right());
                    }
                    Some(result)
                }
            }
            (Direction::U, TileKind::RMirror) | (Direction::D, TileKind::LMirror) => {
                if self.x < max_x {
                    Some(vec![self.right()])
                } else {
                    None
                }
            }
            (Direction::U, TileKind::LMirror) | (Direction::D, TileKind::RMirror) => {
                if self.x > 0 {
                    Some(vec![self.left()])
                } else {
                    None
                }
            }
            (Direction::D, TileKind::Empty) | (Direction::D, TileKind::VSplitter) => {
                if self.y == max_y {
                    None
                } else {
                    Some(vec![self.down()])
                }
            }
            (Direction::L, TileKind::Empty) | (Direction::L, TileKind::HSplitter) => {
                if self.x > 0 {
                    Some(vec![self.left()])
                } else {
                    None
                }
            }
            (Direction::L, TileKind::VSplitter) | (Direction::R, TileKind::VSplitter) => {
                if tile.is_energized() {
                    None
                } else {
                    let mut result = vec![];
                    if self.y > 0 {
                        result.push(self.up());
                    }
                    if self.y < max_y {
                        result.push(self.down());
                    }
                    Some(result)
                }
            }
            (Direction::L, TileKind::LMirror) | (Direction::R, TileKind::RMirror) => {
                if self.y > 0 {
                    Some(vec![self.up()])
                } else {
                    None
                }
            }
            (Direction::L, TileKind::RMirror) | (Direction::R, TileKind::LMirror) => {
                if self.y < max_y {
                    Some(vec![self.down()])
                } else {
                    None
                }
            }
            (Direction::R, TileKind::Empty) | (Direction::R, TileKind::HSplitter) => {
                if self.x < max_x {
                    Some(vec![self.right()])
                } else {
                    None
                }
            }
        }
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn send_light_beam(&mut self, first_light_beam: LightBeam) {
        let max_x = self.tiles[0].len() - 1;
        let max_y = self.tiles.len() - 1;
        let mut light_beams = vec![first_light_beam];
        while let Some(light_beam) = light_beams.pop() {
            if let Some(mut next_beams) =
                light_beam.next(&self.tiles[light_beam.y][light_beam.x], max_x, max_y)
            {
                light_beams.append(&mut next_beams);
            }
            self.tiles[light_beam.y][light_beam.x].energize();
        }
    }

    fn count_energized_tiles(&self) -> usize {
        self.tiles
            .iter()
            .map(|tiles| tiles.iter().filter(|tile| tile.is_energized()).count())
            .sum()
    }

    fn reset(&mut self) {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        for y in 0..height {
            for x in 0..width {
                self.tiles[y][x].de_energize();
            }
        }
    }

    fn find_max_energy(&mut self) -> usize {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        self.reset();
        let mut max = 0;
        // From the top
        for x in 0..width {
            self.reset();
            self.send_light_beam(LightBeam {
                direction: Direction::D,
                x,
                y: 0,
            });
            max = max.max(self.count_energized_tiles());
        }
        // From the bottom
        for x in 0..width {
            self.reset();
            self.send_light_beam(LightBeam {
                direction: Direction::U,
                x,
                y: height - 1,
            });
            max = max.max(self.count_energized_tiles());
        }
        // From the left
        for y in 0..height {
            self.reset();
            self.send_light_beam(LightBeam {
                direction: Direction::R,
                x: 0,
                y: y,
            });
            max = max.max(self.count_energized_tiles());
        }
        // From the right
        for y in 0..height {
            self.reset();
            self.send_light_beam(LightBeam {
                direction: Direction::L,
                x: width - 1,
                y: y,
            });
            max = max.max(self.count_energized_tiles());
        }
        max
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(|c| Tile::from(c)).collect())
            .collect();
        Self { tiles }
    }
}

fn main() {
    let input = include_str!("../../input/day-16");
    let mut grid = Grid::from(input);
    grid.send_light_beam(LightBeam {
        direction: Direction::R,
        x: 0,
        y: 0,
    });
    let nb_energized_tiles = grid.count_energized_tiles();
    println!("{} tiles are energized", nb_energized_tiles);

    grid.reset();
    let max_energy = grid.find_max_energy();
    println!("Maximum energy from the grid: {}", max_energy);
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Grid, LightBeam};

    #[test]
    fn send_beam() {
        let input = include_str!("../../input/day-16-test");
        let mut grid = Grid::from(input);
        grid.send_light_beam(LightBeam {
            direction: Direction::R,
            x: 0,
            y: 0,
        });
        assert_eq!(grid.count_energized_tiles(), 46);
    }

    #[test]
    fn send_multiple_beams() {
        let input = include_str!("../../input/day-16-test");
        let mut grid = Grid::from(input);
        assert_eq!(grid.find_max_energy(), 51);
    }
}
