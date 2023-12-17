struct Pattern {
    vertical: Vec<String>,
    horizontal: Vec<String>,
}

impl Pattern {
    fn add_reflections(&self) -> usize {
        Pattern::get_reflection(&self.vertical, false).unwrap_or_default()
            + 100 * Pattern::get_reflection(&self.horizontal, false).unwrap_or_default()
    }

    fn add_reflections_with_smudge(&self) -> usize {
        Pattern::get_reflection(&self.vertical, true).unwrap_or_default()
            + 100 * Pattern::get_reflection(&self.horizontal, true).unwrap_or_default()
    }

    fn get_reflection(array: &[String], smudge: bool) -> Option<usize> {
        let threshold = if smudge { 1 } else { 0 };
        let mut result = None;
        let len = array.len();
        for index in 0..array.len() - 1 {
            if Pattern::number_of_different_chars(&array[index], &array[index + 1]) <= threshold {
                let number_of_differences: usize = (0..=index)
                    .rev()
                    .zip(index + 1..len)
                    .map(|(a, b)| Pattern::number_of_different_chars(&array[a], &array[b]))
                    .sum();
                if number_of_differences == threshold {
                    result = Some(index + 1);
                    break;
                }
            }
        }
        result
    }

    fn number_of_different_chars(str1: &str, str2: &str) -> usize {
        str1.chars()
            .zip(str2.chars())
            .filter(|(c1, c2)| c1 != c2)
            .count()
    }
}

impl From<&str> for Pattern {
    fn from(input: &str) -> Self {
        let horizontal: Vec<String> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();
        let width = horizontal[0].len();
        let height = horizontal.len();
        let mut vertical = vec![String::with_capacity(height); width];
        input.lines().for_each(|line| {
            line.char_indices()
                .for_each(|(index, c)| vertical[index].push(c))
        });
        Self {
            vertical,
            horizontal,
        }
    }
}

struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    fn sum_of_reflections(&self) -> usize {
        self.patterns
            .iter()
            .map(|pattern| pattern.add_reflections())
            .sum()
    }

    fn sum_of_reflections_with_smudge(&self) -> usize {
        self.patterns
            .iter()
            .map(|pattern| pattern.add_reflections_with_smudge())
            .sum()
    }
}

impl From<&str> for Patterns {
    fn from(input: &str) -> Self {
        let mut patterns = vec![];
        let mut string = String::new();
        input.lines().for_each(|line| {
            if line.is_empty() {
                let pattern = Pattern::from(string.as_str());
                patterns.push(pattern);
                string = String::new()
            } else {
                string.push_str(line);
                string.push('\n');
            }
        });
        let pattern = Pattern::from(string.as_str());
        patterns.push(pattern);
        Patterns { patterns }
    }
}

fn main() {
    let input = include_str!("../../input/day-13");
    let patterns = Patterns::from(input);
    let sum = patterns.sum_of_reflections();
    println!("Sum: {}", sum);
    let sum_with_smudge = patterns.sum_of_reflections_with_smudge();
    println!("Sum with smudge: {}", sum_with_smudge);
}

#[cfg(test)]
mod tests {
    use crate::Patterns;

    #[test]
    fn reflections() {
        let input = include_str!("../../input/day-13-test");
        let patterns = Patterns::from(input);
        assert_eq!(patterns.sum_of_reflections(), 405);
    }

    #[test]
    fn reflections_with_smudge() {
        let input = include_str!("../../input/day-13-test");
        let patterns = Patterns::from(input);
        assert_eq!(patterns.sum_of_reflections_with_smudge(), 400);
    }
}
