use advent_of_code::utils::{parse_input_by_lines, Parsable};
use itertools::Itertools;
use nom::character::complete::space1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(1);

pub fn parse(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let parse_line = separated_pair(i64::parse, space1, i64::parse);
    parse_input_by_lines(parse_line)(input)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, input) = parse(input).unwrap();

    let left: Vec<_> = input.iter().map(|(l, _)| l).sorted().collect();
    let right: Vec<_> = input.iter().map(|(_, r)| r).sorted().collect();

    Some(
        left.into_iter()
            .zip(right.into_iter())
            .map(|(&l, &r)| (l - r).abs())
            .sum::<i64>(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, input) = parse(input).unwrap();

    let left: Vec<_> = input.iter().map(|(l, _)| l).collect();
    let right: Vec<_> = input.iter().map(|(_, r)| r).collect();

    let counts = right.into_iter().counts();

    Some(
        left.into_iter()
            .map(|&l| l * *counts.get(&l).unwrap_or(&0) as i64)
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
            Ok(("", vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)]))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
