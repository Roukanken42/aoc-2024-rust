use crate::PrefixSize::{Partial, Whole};
use advent_of_code::utils::{end_of_file, Parsable};
use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

advent_of_code::solution!(2);

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, reports) = separated_list1(line_ending, Vec::parse)(input)?;
    let (input, _) = end_of_file(input)?;
    Ok((input, reports))
}

#[derive(Debug, PartialEq)]
enum PrefixSize {
    Whole,
    Partial(usize),
}

fn find_valid_report_prefix<'a>(mut report: impl Iterator<Item=(&'a i32, &'a i32)>, increasing: bool) -> PrefixSize {
    report
        .find_position(|(&a, &b)| (b - a).abs() > 3 || (b - a).abs() < 1 || (a < b) != increasing)
        .map(|(i, _)| Partial(i + 1))
        .unwrap_or(Whole)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, reports) = parse(input).unwrap();

    let valid_reports = reports
        .into_iter()
        .filter(|report| {
            find_valid_report_prefix(report.iter().tuple_windows(), true) == Whole
                || find_valid_report_prefix(report.iter().tuple_windows(), false) == Whole
        })
        .count();

    Some(valid_reports)
}

fn is_valid_report_with_replacement(report: &[i32], increasing: bool) -> bool {
    let prefix_front = find_valid_report_prefix(report.iter().tuple_windows(), increasing);
    let prefix_back = find_valid_report_prefix(report.iter().rev().tuple_windows(), !increasing);

    let Partial(front) = prefix_front else { return true };
    let Partial(back) = prefix_back else { return true };

    // If we can remove the front or back element and the report is valid afterwards
    if back == report.len() - 1 || front == report.len() - 1 {
        return true;
    }

    // If we remove the first invalid element from front
    let diff = report[front + 1] - report[front - 1];
    if diff.abs() <= 3 && diff.abs() >= 1 && (diff > 0) == increasing && front + back + 1 >= report.len() {
        return true;
    }

    // If we remove the last valid element from front
    if front > 1 {
        let diff = report[front] - report[front - 2];
        if diff.abs() <= 3 && diff.abs() >= 1 && (diff > 0) == increasing && front + back >= report.len() {
            return true;
        }
    }

    false
}
pub fn part_two(input: &str) -> Option<usize> {
    let (_, reports) = parse(input).unwrap();

    let valid_reports = reports
        .into_iter()
        .filter(|report| is_valid_report_with_replacement(&report, true) || is_valid_report_with_replacement(&report, false))
        .count();

    Some(valid_reports)
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
    fn test_find_valid_report_prefix() {
        let report = vec![1, 3, 6, 7, 9];
        assert_eq!(find_valid_report_prefix(report.iter().tuple_windows(), true), Whole);
        assert_eq!(find_valid_report_prefix(report.iter().rev().tuple_windows(), false), Whole);

        let report = vec![1, 2, 7, 8, 9];
        assert_eq!(find_valid_report_prefix(report.iter().tuple_windows(), true), Partial(2));
        assert_eq!(find_valid_report_prefix(report.iter().rev().tuple_windows(), false), Partial(3));
    }

    #[test]
    fn test_is_valid_report_with_replacement() {
        let report = vec![72, 73, 75, 77, 79, 82, 79, 85];
        assert_eq!(is_valid_report_with_replacement(&report, true), true);

        let report = vec![60, 61, 64, 64, 67, 69];
        assert_eq!(is_valid_report_with_replacement(&report, true), true);

        let report = vec![30, 32, 33, 35, 38, 42, 41];
        assert_eq!(is_valid_report_with_replacement(&report, true), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
