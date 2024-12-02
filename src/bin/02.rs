use advent_of_code::utils::{end_of_file, Parsable};
use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, reports) = separated_list1(line_ending, Vec::parse)(input)?;
    let (input, _) = end_of_file(input)?;
    Ok((input, reports))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, reports) = parse(input).unwrap();

    let positive_gradual_diffs = HashSet::from([1, 2, 3]);
    let negative_gradual_diffs = HashSet::from([-1, -2, -3]);

    let valid_reports = reports.into_iter().filter(|report| {
        let diffs = report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| a - b)
            .counts()
            .keys()
            .copied()
            .collect::<HashSet<_>>();

        diffs.is_subset(&positive_gradual_diffs) || diffs.is_subset(&negative_gradual_diffs)
    }).count();

    Some(valid_reports)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
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
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9],
                ]
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
