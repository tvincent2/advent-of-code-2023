use std::{collections::HashMap, ops::Deref};

#[derive(Debug, PartialEq)]
struct Record<const N: usize> {
    // only damaged or unknown springs
    springs: Vec<String>,
    criteria: Vec<usize>,
}

impl<const N: usize> Record<N> {
    fn count_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        self.count_arrangements_rec(0, 0, 0, &mut cache)
    }

    fn count_arrangements_rec(
        &self,
        s_index: usize,
        s_taken: usize,
        c_index: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(&result) = cache.get(&(s_index, s_taken, c_index)) {
            return result;
        }
        let springs = &self.springs;
        let criteria = &self.criteria;

        let result = if s_index < springs.len() {
            let current_spring_set = springs[s_index].as_str();
            if s_taken == current_spring_set.len() {
                self.count_arrangements_rec(s_index + 1, 0, c_index, cache)
            } else if s_taken > current_spring_set.len() {
                0
            } else if c_index < criteria.len() {
                let current_criteria = criteria[c_index];
                if current_criteria > current_spring_set.len() - s_taken {
                    if current_spring_set[s_taken..].contains("#") {
                        0
                    } else {
                        self.count_arrangements_rec(s_index + 1, 0, c_index, cache)
                    }
                } else if current_criteria == current_spring_set.len() - s_taken {
                    if current_spring_set[s_taken..].contains("#") {
                        self.count_arrangements_rec(s_index + 1, 0, c_index + 1, cache)
                    } else {
                        self.count_arrangements_rec(s_index + 1, 0, c_index + 1, cache)
                            + self.count_arrangements_rec(s_index + 1, 0, c_index, cache)
                    }
                } else {
                    let next_char_index = s_taken + current_criteria;
                    if &current_spring_set[next_char_index..next_char_index + 1] == "#" {
                        if &current_spring_set[s_taken..s_taken + 1] == "#" {
                            0
                        } else {
                            self.count_arrangements_rec(s_index, s_taken + 1, c_index, cache)
                        }
                    } else if &current_spring_set[s_taken..s_taken + 1] == "#" {
                        self.count_arrangements_rec(
                            s_index,
                            s_taken + current_criteria + 1,
                            c_index + 1,
                            cache,
                        )
                    } else {
                        self.count_arrangements_rec(
                            s_index,
                            s_taken + current_criteria + 1,
                            c_index + 1,
                            cache,
                        ) + self.count_arrangements_rec(s_index, s_taken + 1, c_index, cache)
                    }
                }
            } else {
                if springs[s_index][s_taken..].contains("#") {
                    0
                } else {
                    self.count_arrangements_rec(s_index + 1, 0, c_index, cache)
                }
            }
        } else {
            if c_index == criteria.len() {
                1
            } else {
                0
            }
        };
        cache.insert((s_index, s_taken, c_index), result);
        result
    }
}

impl<const N: usize> From<&str> for Record<N> {
    fn from(input: &str) -> Self {
        let mut split = input.split(" ");
        let springs_str = split.next().expect("missing spring group data");
        let mut more_springs = String::with_capacity(springs_str.len() * N + N - 1);
        more_springs.push_str(springs_str);
        (2..=N).for_each(|_| {
            more_springs.push('?');
            more_springs.push_str(springs_str);
        });
        let springs = more_springs
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let criteria = std::iter::repeat(
            split
                .next()
                .expect("missing criteria data")
                .split(",")
                .map(|input| input.parse::<usize>().expect("unparseable criteria")),
        )
        .take(N)
        .flatten()
        .collect();
        Self { springs, criteria }
    }
}

struct Records<const N: usize> {
    records: Vec<Record<N>>,
}

impl<const N: usize> Records<N> {
    fn arrangements(&self) -> Vec<usize> {
        self.iter()
            .map(|record| record.count_arrangements())
            .collect()
    }

    fn sum_arrangements(&self) -> usize {
        self.arrangements().iter().sum()
    }
}

impl<const N: usize> From<&str> for Records<N> {
    fn from(input: &str) -> Self {
        let records = input.lines().map(|line| Record::from(line)).collect();
        Self { records }
    }
}

impl<const N: usize> Deref for Records<N> {
    type Target = Vec<Record<N>>;

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

fn main() {
    let input = include_str!("../../input/day-12");
    let records = Records::<1>::from(input);
    let sum_of_arrangements = records.sum_arrangements();
    println!("Sum of arragements: {}", sum_of_arrangements);

    let records5 = Records::<5>::from(input);
    let sum_of_arrangements5 = records5.sum_arrangements();
    println!("Sum of arragements: {}", sum_of_arrangements5);
}

#[cfg(test)]
mod tests {
    use crate::{Record, Records};

    #[test]
    fn parse_record() {
        let input = "???.### 1,1,3";
        let record = Record::<1>::from(input);
        assert_eq!(
            record,
            Record {
                springs: vec!["???".to_string(), "###".to_string()],
                criteria: vec![1, 1, 3]
            }
        );
    }

    #[test]
    fn count() {
        let input = "???.### 1,1,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count2() {
        let input = ".??..??...?##. 1,1,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn count3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count3_mini() {
        let input = "?#?#?#? 1,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count4() {
        let input = "?###???????? 3,2,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 10);
    }

    #[test]
    fn count4_mini() {
        let input = "?###? 3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count4_mini2() {
        let input = "?###?????? 3,2,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 3);
    }

    #[test]
    fn count4_mini3() {
        let input = "?###??????? 3,2,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 6);
    }

    #[test]
    fn sum_of_arrangements() {
        let input = include_str!("../../input/day-12-test");
        let records = Records::<1>::from(input);
        assert_eq!(records.sum_arrangements(), 21);
    }

    #[test]
    fn arrangements() {
        let input = include_str!("../../input/day-12-test");
        let records = Records::<1>::from(input);
        let arrangements: Vec<usize> = records
            .iter()
            .map(|record| record.count_arrangements())
            .collect();
        assert_eq!(arrangements, vec![1, 4, 1, 1, 4, 10]);
    }

    #[test]
    fn count_from_real_data() {
        let input = "?#?##?#????.?..?? 9,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 7);
    }

    #[test]
    fn count_from_real_data2() {
        let input = "????????#???#? 1,8";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 9);
    }

    #[test]
    fn count_from_real_data3() {
        let input = ".??????#???#??????? 2,7,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 50);
    }

    #[test]
    fn count_from_real_data3_mini() {
        let input = ".??#???#??????? 7,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 19);
    }

    #[test]
    fn count_from_real_data3_mini_bis() {
        let input = "??#???#??????? 7,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 19);
    }

    #[test]
    fn count_from_real_data3_mini2() {
        let input = "#?#???#??????? 7,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 10);
    }

    #[test]
    fn count_from_real_data3_mini3() {
        let input = "?????? 1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 10);
    }

    #[test]
    fn count_from_real_data4() {
        let input = "???.?.?.???#.? 1,1,1,4";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 5);
    }

    #[test]
    fn count_from_real_data5() {
        let input = "??#???#?##??. 4,5";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 3);
    }

    #[test]
    fn count_from_real_data6() {
        let input = "??????##??#???????? 1,10,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 50);
    }

    #[test]
    fn count_from_real_data7() {
        let input = "?#?#??.???.???? 6,1,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 10);
    }

    #[test]
    fn count_from_real_data8() {
        let input = "?#?#??..#???#???#? 6,1,4,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 3);
    }

    #[test]
    fn count_from_real_data9() {
        let input = "??????.?.???#?####? 5,1,9";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn count_from_real_data10() {
        let input = ".#?#??#.????#?#??#? 4,1,1,5,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 3);
    }

    #[test]
    fn count_from_real_data11() {
        let input = "????..????..??? 3,2,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 12);
    }

    #[test]
    fn count_from_real_data12() {
        let input = ".?#??#.??.? 4,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data13() {
        let input = "???#????#??? 2,4";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 7);
    }

    #[test]
    fn count_from_real_data14() {
        let input = "#.??#??#??.????#? 1,3,3,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 9);
    }

    #[test]
    fn count_from_real_data15() {
        let input = "???.#?????#. 1,1,4,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data16() {
        let input = "??#?#?.??#??. 3,1,1,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data17() {
        let input = "?????#???? 2,4,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn count_from_real_data18() {
        let input = "?.??????.?#?#? 5,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data19() {
        let input = ".?????#???..? 7,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn count_from_real_data20() {
        let input = ".????????.# 1,3,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 10);
    }

    #[test]
    fn count_from_real_data21() {
        let input = "???#.?.?.. 1,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 5);
    }

    #[test]
    fn count_from_real_data22() {
        let input = "?.???.??????#??#??? 2,2,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 19);
    }

    #[test]
    fn count_from_real_data22_mini() {
        let input = "?.?#?.??????#??#??? 2,2,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 18);
    }

    #[test]
    fn count_from_real_data22_mini2() {
        let input = "?.?.?.??????#??#??? 2,2,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data22_mini3() {
        let input = "??????#??#??? 2,2,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data23() {
        let input = "??#???.???????#??# 2,10";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data24() {
        let input = "?#??#?.????.???? 2,2,1,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 72);
    }

    #[test]
    fn count_from_real_data24_mini() {
        let input = "?#??#? 2,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 3);
    }

    #[test]
    fn count_from_real_data24_mini2() {
        let input = "????.???? 1,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 24);
    }

    #[test]
    fn count_from_real_data25() {
        let input = "????#?##??. 1,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 6);
    }

    #[test]
    fn count_from_real_data26() {
        let input = "#?#???#???.????? 8,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 6);
    }

    #[test]
    fn count_from_real_data27() {
        let input = "???.?????#???? 1,7";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 15);
    }

    #[test]
    fn count_from_real_data28() {
        let input = "?????#?????#?...???# 11,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn count_from_real_data29() {
        let input = "?.?#?.??.#?#? 2,2,4";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data30() {
        let input = "???#.????#??##. 1,1,3,1,2";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data31() {
        let input = "??????.????? 2,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 34);
    }

    #[test]
    fn count_from_real_data32() {
        let input = "???#???#?..??.?#??## 1,7,2,6";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data33() {
        let input = "?.??#..#???#???.?? 1,1,5,1,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 8);
    }

    #[test]
    fn count_from_real_data34() {
        let input = "?.???###?? 1,5,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data35() {
        let input = "???.?.#??#?????.?#? 3,2,1,3,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data36() {
        let input = "???.??.???? 1,1,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 14);
    }

    #[test]
    fn count_from_real_data37() {
        let input = ".?##????????.. 6,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 7);
    }

    #[test]
    fn count_from_real_data38() {
        let input = "????????#???##?#?#. 1,2,2,8,1";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn count_from_real_data39() {
        let input = "?##?#?????#.????? 4,1,2,4";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn count_from_real_data71() {
        let input = "???#??#?#..???? 2,3,4";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data71_mini() {
        let input = "???#??#?# 2,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data71_mini2() {
        let input = "..?#??#?# 2,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn count_from_real_data71_mini3() {
        let input = ".??#??#?# 2,3";
        let record = Record::<1>::from(input);
        assert_eq!(record.count_arrangements(), 2);
    }

    #[test]
    fn parse_record_part2() {
        let input = "???.### 1,1,3";
        let record = Record::<5>::from(input);
        assert_eq!(
            record,
            Record {
                springs: vec![
                    "???".to_string(),
                    "###????".to_string(),
                    "###????".to_string(),
                    "###????".to_string(),
                    "###????".to_string(),
                    "###".to_string()
                ],
                criteria: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            }
        );
    }

    #[test]
    fn count_part2() {
        let input = "???.### 1,1,3";
        let record = Record::<5>::from(input);
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn arrangements_part2() {
        let input = include_str!("../../input/day-12-test");
        let records2 = Records::<5>::from(input);

        let sum = records2.sum_arrangements();
        assert_eq!(sum, 525152);
    }
}
