use std::ops::Deref;

trait Possible {
    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool;
}

struct CubeGrab {
    red: usize,
    blue: usize,
    green: usize,
}

impl Possible for CubeGrab {
    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        r >= self.red && g >= self.green && b >= self.blue
    }
}

impl From<&str> for CubeGrab {
    fn from(input: &str) -> Self {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        input.split(",").for_each(|grab| {
            let split: Vec<&str> = grab.split(" ").collect();
            let value = split[1].parse::<usize>().unwrap();
            match split[2] {
                "blue" => blue += value,
                "green" => green += value,
                "red" => red += value,
                _ => unreachable!(),
            }
        });
        Self { red, blue, green }
    }
}

struct Game {
    id: usize,
    cube_grabs: Vec<CubeGrab>,
}

impl Possible for Game {
    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        self.cube_grabs
            .iter()
            .all(|cube_grab| cube_grab.is_possible(r, g, b))
    }
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let split: Vec<&str> = input.split(":").collect();
        let id_input: Vec<&str> = split[0].split(" ").collect();
        let grabs_input = split[1].split(";");
        let id = id_input[1].parse::<usize>().unwrap();
        Self {
            id,
            cube_grabs: grabs_input.map(|grab_input| grab_input.into()).collect(),
        }
    }
}

impl Game {
    fn power(&self) -> usize {
        let max_red = self
            .cube_grabs
            .iter()
            .map(|grab| grab.red)
            .max()
            .unwrap_or_default();
        let max_blue = self
            .cube_grabs
            .iter()
            .map(|grab| grab.blue)
            .max()
            .unwrap_or_default();
        let max_green = self
            .cube_grabs
            .iter()
            .map(|grab| grab.green)
            .max()
            .unwrap_or_default();
        max_red * max_blue * max_green
    }
}

struct Games {
    games: Vec<Game>,
}

impl From<&str> for Games {
    fn from(input: &str) -> Self {
        Self {
            games: input.lines().map(|game_input| game_input.into()).collect(),
        }
    }
}

impl Deref for Games {
    type Target = Vec<Game>;

    fn deref(&self) -> &Self::Target {
        &self.games
    }
}

impl Games {
    fn sum_of_possible_ids(&self, r: usize, g: usize, b: usize) -> usize {
        self.iter()
            .filter(|game| game.is_possible(r, g, b))
            .map(|game| game.id)
            .sum()
    }

    fn sum_of_powers(&self) -> usize {
        self.iter().map(|game| game.power()).sum()
    }
}

fn main() {
    println!("day 02");
    let input = include_str!("../../input/day-02");
    let games: Games = input.into();

    let result = games.sum_of_possible_ids(12, 13, 14);
    println!("Sum of possible: {}", result);

    let result_2 = games.sum_of_powers();
    println!("Sum of powers: {}", result_2);
}

#[cfg(test)]
mod tests {
    use crate::Games;

    #[test]
    fn test_possible() {
        let input = include_str!("../../input/day-02-test");
        let games: Games = input.into();
        assert_eq!(games.sum_of_possible_ids(12, 13, 14), 8);
    }

    #[test]
    fn test_power() {
        let input = include_str!("../../input/day-02-test");
        let games: Games = input.into();
        assert_eq!(games.sum_of_powers(), 2286);
    }
}
