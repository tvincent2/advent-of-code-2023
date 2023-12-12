use std::ops::Deref;

struct History {
    values: Vec<isize>,
}

impl History {
    fn compute_next_value(&self) -> isize {
        let mut differences: Vec<Vec<isize>> = vec![self.values.clone()];
        loop {
            let last_diff = differences.last().expect("missing diff vec");
            if last_diff.iter().all(|&value| value == 0) {
                break;
            }
            let new_diff = last_diff
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            differences.push(new_diff);
        }
        let mut projections = vec![0; differences.len()];
        for (index, diff) in differences.iter().enumerate().rev().skip(1) {
            projections[index] = diff[diff.len() - 1] + projections[index + 1];
        }
        projections[0]
    }

    fn compute_previous_value(&self) -> isize {
        let mut differences: Vec<Vec<isize>> = vec![self.values.clone()];
        loop {
            let last_diff = differences.last().expect("missing diff vec");
            if last_diff.iter().all(|&value| value == 0) {
                break;
            }
            let new_diff = last_diff
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            differences.push(new_diff);
        }
        let mut projections = vec![0; differences.len()];
        for (index, diff) in differences.iter().enumerate().rev().skip(1) {
            projections[index] = diff[0] - projections[index + 1];
        }
        projections[0]
    }
}

impl From<&str> for History {
    fn from(input: &str) -> Self {
        let values = input
            .split(" ")
            .map(|val| val.parse::<isize>().expect("unparseable value"))
            .collect();
        Self { values }
    }
}

struct Histories {
    histories: Vec<History>,
}

impl Histories {
    fn add_projections(&self) -> isize {
        self.iter()
            .map(|history| history.compute_next_value())
            .sum()
    }

    fn add_prev_projections(&self) -> isize {
        self.iter()
            .map(|history| history.compute_previous_value())
            .sum()
    }
}

impl Deref for Histories {
    type Target = Vec<History>;

    fn deref(&self) -> &Self::Target {
        &self.histories
    }
}

impl From<&str> for Histories {
    fn from(input: &str) -> Self {
        let histories = input.lines().map(|line| History::from(line)).collect();
        Self { histories }
    }
}

fn main() {
    let input = include_str!("../../input/day-09");
    let histories = Histories::from(input);
    let projection = histories.add_projections();
    println!("Sum of projections: {}", projection);
    let prev_projection = histories.add_prev_projections();
    println!("Sum of projections: {}", prev_projection);
}

#[cfg(test)]
mod tests {
    use crate::Histories;

    #[test]
    fn projection() {
        let input = include_str!("../../input/day-09-test");
        let histories = Histories::from(input);
        let projection = histories[0].compute_next_value();
        assert_eq!(projection, 18);
    }

    #[test]
    fn projections() {
        let input = include_str!("../../input/day-09-test");
        let histories = Histories::from(input);
        let projection = histories.add_projections();
        assert_eq!(projection, 114);
    }

    #[test]
    fn prev_projection() {
        let input = include_str!("../../input/day-09-test");
        let histories = Histories::from(input);
        let projection = histories[2].compute_previous_value();
        assert_eq!(projection, 5);
    }

    #[test]
    fn prev_projections() {
        let input = include_str!("../../input/day-09-test");
        let histories = Histories::from(input);
        let projection = histories.add_prev_projections();
        assert_eq!(projection, 2);
    }
}
