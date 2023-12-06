struct CalibrationValue {
    value: usize,
}

fn replace_spelled_numbers(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

impl From<String> for CalibrationValue {
    fn from(value: String) -> Self {
        let result = value
            .split('\n')
            .map(|line| {
                let digits: Vec<usize> = replace_spelled_numbers(line)
                    .chars()
                    .filter_map(|c| c.to_string().parse::<usize>().ok())
                    .collect();
                let result = digits.first().unwrap() * 10 + digits.last().unwrap();
                result
            })
            .sum();

        Self { value: result }
    }
}

fn main() {
    let input = include_str!("../../input/day-01").to_string();
    println!("{:?}", CalibrationValue::from(input).value);
}

#[cfg(test)]
mod tests {
    use crate::CalibrationValue;

    #[test]
    fn test_from() {
        let input = include_str!("../../input/day-01-test").to_string();
        let result = CalibrationValue::from(input);
        assert_eq!(result.value, 142);
    }

    #[test]
    fn test_from_with_spelled_numbers() {
        let input = include_str!("../../input/day-01-test2").to_string();
        let result = CalibrationValue::from(input);
        assert_eq!(result.value, 281);
    }
}
