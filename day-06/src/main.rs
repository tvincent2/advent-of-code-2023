#[derive(Debug, PartialEq)]
struct Races {
    times: Vec<usize>,
    distances: Vec<usize>,
}

impl Races {
    fn count_record_breakers_for_one_race(time: usize, distance: usize) -> usize {
        let delta = time as isize * time as isize - 4 * distance as isize;
        if delta < 0 {
            0
        } else if delta == 0 {
            unimplemented!()
        } else {
            let sqrt = (delta as f64).sqrt();
            let float_solution1 = (time as f64 + sqrt) / 2.;
            let float_solution2 = (time as f64 - sqrt) / 2.;

            let solution1 = if float_solution1.fract() == 0.0 {
                float_solution1 as usize - 1
            } else {
                float_solution1.floor() as usize
            };
            let solution2 = if float_solution2.fract() == 0.0 {
                float_solution2 as usize + 1
            } else {
                float_solution2.ceil() as usize
            };
            solution1 - solution2 + 1
        }
    }

    fn count_record_breakers(&self) -> usize {
        let record_breakers: Vec<usize> = self
            .times
            .iter()
            .zip(self.distances.iter())
            .map(|(time, distance)| Races::count_record_breakers_for_one_race(*time, *distance))
            .collect();
        let mut result = 1;
        for rb in record_breakers {
            result *= rb;
        }
        result
    }
}

impl From<&str> for Races {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let times = lines
            .next()
            .expect("missing Time line")
            .split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("unparseable number"))
            .collect();
        let distances = lines
            .next()
            .expect("missing Distance line")
            .split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("unparseable number"))
            .collect();
        Self { times, distances }
    }
}

fn main() {
    let input = include_str!("../../input/day-06");
    let races = Races::from(input);
    let record_breakers_result = races.count_record_breakers();
    println!("Record breakers: {}", record_breakers_result);

    let input2 = include_str!("../../input/day-06-part2");
    let races2 = Races::from(input2);
    let record_breakers_result2 = races2.count_record_breakers();
    println!("Record breakers: {}", record_breakers_result2);
}

#[cfg(test)]
mod tests {
    use crate::Races;

    #[test]
    fn parse() {
        let input = include_str!("../../input/day-06-test");
        let races = Races::from(input);
        assert_eq!(
            races,
            Races {
                times: vec![7, 15, 30],
                distances: vec![9, 40, 200]
            }
        );
    }

    #[test]
    fn record_breakers_for_one_race() {
        assert_eq!(Races::count_record_breakers_for_one_race(7, 9), 4);
        assert_eq!(Races::count_record_breakers_for_one_race(15, 40), 8);
        assert_eq!(Races::count_record_breakers_for_one_race(30, 200), 9);
    }

    #[test]
    fn record_breakers() {
        let input = include_str!("../../input/day-06-test");
        let races = Races::from(input);
        let result = races.count_record_breakers();

        assert_eq!(result, 288);
    }
}
