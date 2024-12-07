use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::bytes::complete::tag;
use nom::IResult;
use std::collections::HashSet;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Parsable<'_> for Equation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, result) = u64::parse(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, numbers) = Vec::parse(input)?;

        Ok((input, Equation { result, numbers }))
    }
}

impl Equation {
    #[allow(dead_code)]
    fn is_solvable(&self) -> bool {
        let mut possible_values = HashSet::from([self.numbers[0]]);

        for number in self.numbers.iter().skip(1) {
            let mut new_possible_values = HashSet::new();
            for value in &possible_values {
                new_possible_values.insert(value + number);
                new_possible_values.insert(value * number);
            }
            possible_values = new_possible_values;
        }

        possible_values.contains(&self.result)
    }

    fn is_solvable_backwards(&self) -> bool {
        let mut possible_values = HashSet::from([self.result]);

        for number in self.numbers.iter().rev() {
            let mut new_possible_values = HashSet::new();

            for value in &possible_values {
                if value >= number {
                    new_possible_values.insert(value - number);
                }
                if value % number == 0 {
                    new_possible_values.insert(value / number);
                }
            }
            possible_values = new_possible_values;
        }

        possible_values.contains(&0)
    }

    #[allow(dead_code)]
    fn is_solvable_with_concat(&self) -> bool {
        let mut possible_values = HashSet::from([self.numbers[0]]);

        for number in self.numbers.iter().skip(1) {
            let mut new_possible_values = HashSet::new();
            for value in &possible_values {
                new_possible_values.insert(value + number);
                new_possible_values.insert(value * number);
                new_possible_values.insert(value * 10u64.pow(number.ilog10() + 1) + number);
            }
            possible_values = new_possible_values;
        }

        possible_values.contains(&self.result)
    }

    fn is_solvable_backwards_with_concat(&self) -> bool {
        let mut possible_values = HashSet::from([self.result]);

        for number in self.numbers.iter().rev() {
            let mut new_possible_values = HashSet::new();

            for value in &possible_values {
                if value >= number {
                    new_possible_values.insert(value - number);
                }
                if value % number == 0 {
                    new_possible_values.insert(value / number);
                }

                let concat = 10u64.pow(number.ilog10() + 1);
                if *value % concat == *number {
                    new_possible_values.insert(*value / concat);
                }
            }
            possible_values = new_possible_values;
        }

        possible_values.contains(&0)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    parse_input_by_lines(Equation::parse)(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, equations) = parse(input).unwrap();

    Some(
        equations
            .into_iter()
            .filter(|equation| equation.is_solvable_backwards())
            .map(|equation| equation.result)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, equations) = parse(input).unwrap();

    Some(
        equations
            .into_iter()
            .filter(|equation| equation.is_solvable_backwards_with_concat())
            .map(|equation| equation.result)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);

        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Equation { result: 190, numbers: vec![10, 19] },
                    Equation { result: 3267, numbers: vec![81, 40, 27] },
                    Equation { result: 83, numbers: vec![17, 5] },
                    Equation { result: 156, numbers: vec![15, 6] },
                    Equation { result: 7290, numbers: vec![6, 8, 6, 15] },
                    Equation { result: 161011, numbers: vec![16, 10, 13] },
                    Equation { result: 192, numbers: vec![17, 8, 14] },
                    Equation { result: 21037, numbers: vec![9, 7, 18, 13] },
                    Equation { result: 292, numbers: vec![11, 6, 16, 20] },
                ]
            ))
        );
    }

    #[test]
    fn test_is_solvable() {
        assert_eq!(Equation { result: 3267, numbers: vec![81, 40, 27] }.is_solvable(), true);
        assert_eq!(Equation { result: 156, numbers: vec![15, 6] }.is_solvable(), false);
        assert_eq!(Equation { result: 83, numbers: vec![17, 5] }.is_solvable(), false);
    }

    #[test]
    fn test_is_solvable_backwards() {
        assert_eq!(Equation { result: 3267, numbers: vec![81, 40, 27] }.is_solvable_backwards(), true);
        assert_eq!(Equation { result: 156, numbers: vec![15, 6] }.is_solvable_backwards(), false);
        assert_eq!(Equation { result: 83, numbers: vec![17, 5] }.is_solvable_backwards(), false);
    }

    #[test]
    fn test_is_solvable_with_concat() {
        assert_eq!(Equation { result: 3267, numbers: vec![81, 40, 27] }.is_solvable_with_concat(), true);
        assert_eq!(Equation { result: 156, numbers: vec![15, 6] }.is_solvable_with_concat(), true);
        assert_eq!(Equation { result: 83, numbers: vec![17, 5] }.is_solvable_with_concat(), false);
    }

    #[test]
    fn test_is_solvable_backwards_with_concat() {
        assert_eq!(Equation { result: 3267, numbers: vec![81, 40, 27] }.is_solvable_backwards_with_concat(), true);
        assert_eq!(Equation { result: 156, numbers: vec![15, 6] }.is_solvable_backwards_with_concat(), true);
        assert_eq!(Equation { result: 83, numbers: vec![17, 5] }.is_solvable_backwards_with_concat(), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
