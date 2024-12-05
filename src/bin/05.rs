use advent_of_code::utils::{end_of_file, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct Rule {
    lesser: u32,
    greater: u32,
}

impl Parsable<'_> for Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        let pair = separated_pair(u32::parse, tag("|"), u32::parse);
        map(pair, |(lesser, greater)| Rule { lesser, greater })(input)
    }
}

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Vec<u32>>,
}

impl Parsable<'_> for Input {
    fn parse(input: &'_ str) -> IResult<&'_ str, Self> {
        let (input, rules) = separated_list1(line_ending, Rule::parse)(input)?;
        let (input, _) = many1(line_ending)(input)?;

        let parse_update = separated_list1(tag(","), u32::parse);
        let (input, updates) = separated_list1(line_ending, parse_update)(input)?;

        let (input, _) = end_of_file(input)?;

        Ok((input, Input { rules, updates }))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = Input::parse(&input);

        let (str, input) = result.unwrap();

        assert!(str.is_empty());
        assert_eq!(
            input.rules[0],
            Rule {
                lesser: 47,
                greater: 53
            }
        );
        assert_eq!(
            input.rules.last(),
            Some(&Rule {
                lesser: 53,
                greater: 13
            })
        );

        assert_eq!(input.updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
