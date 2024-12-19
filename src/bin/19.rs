use advent_of_code::utils::{end_of_file, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, newline};
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use Color::{Black, Blue, Green, Red, White};

advent_of_code::solution!(19);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Parsable<'_> for Color {
    fn parse(input: &'_ str) -> IResult<&'_ str, Self> {
        alt((
            value(White, char('w')),
            value(Blue, char('u')),
            value(Black, char('b')),
            value(Red, char('r')),
            value(Green, char('g')),
        ))(input)
    }
}

struct Input {
    towels: Vec<Vec<Color>>,
    patterns: Vec<Vec<Color>>,
}

impl Parsable<'_> for Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, towels) = separated_list1(tag(", "), many1(Color::parse))(input)?;
        let (input, _) = many1(line_ending)(input)?;

        let (input, patterns) = separated_list1(newline, many1(Color::parse))(input)?;
        let (input, _) = end_of_file(input)?;

        Ok((input, Input { towels, patterns }))
    }
}

fn is_pattern_possible(towels: &[Vec<Color>], pattern: &[Color]) -> bool {
    let mut possible = vec![false; pattern.len() + 1];
    possible[0] = true;

    for i in 0..possible.len() {
        if !possible[i] {
            continue;
        }

        'towel: for towel in towels {
            if *possible.get(i + towel.len()).unwrap_or(&true) {
                continue 'towel;
            }

            if towel[..] == pattern[i..i + towel.len()] {
                possible[i + towel.len()] = true;
            }
        }
    }

    possible[possible.len() - 1]
}

fn count_patterns(towels: &[Vec<Color>], pattern: &[Color]) -> u64 {
    let mut possible = vec![0; pattern.len() + 1];
    possible[0] = 1;

    for i in 0..possible.len() {
        if !possible[i] == 0 {
            continue;
        }

        for towel in towels {
            if i + towel.len() >= possible.len() {
                continue;
            }

            if towel[..] == pattern[i..i + towel.len()] {
                possible[i + towel.len()] += possible[i];
            }
        }
    }

    possible[possible.len() - 1]
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = Input::parse(input).unwrap();

    Some(
        input
            .patterns
            .iter()
            .filter(|pattern| is_pattern_possible(&input.towels, pattern))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = Input::parse(input).unwrap();

    Some(input.patterns.iter().map(|pattern| count_patterns(&input.towels, pattern)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, result) = Input::parse(&input).unwrap();

        assert_eq!(rest, "");
        assert_eq!(
            result.towels,
            vec![
                vec![Red],
                vec![White, Red],
                vec![Black],
                vec![Green],
                vec![Black, White, Blue],
                vec![Red, Black],
                vec![Green, Black],
                vec![Black, Red],
            ],
        );

        assert_eq!(
            result.patterns[..=2],
            vec![
                vec![Black, Red, White, Red, Red],
                vec![Black, Green, Green, Red],
                vec![Green, Black, Black, Red],
            ],
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
