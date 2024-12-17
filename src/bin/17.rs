use advent_of_code::utils::{end_of_file, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

advent_of_code::solution!(17);

#[derive(Debug, PartialEq)]
struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    memory: Vec<u32>,
}

impl Parsable<'_> for Computer {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, reg_a) = delimited(tag("Register A: "), u32::parse, line_ending)(input)?;
        let (input, reg_b) = delimited(tag("Register B: "), u32::parse, line_ending)(input)?;
        let (input, reg_c) = delimited(tag("Register C: "), u32::parse, line_ending)(input)?;

        let (input, _) = many1(line_ending)(input)?;
        let (input, memory) = preceded(tag("Program: "), separated_list1(tag(","), u32::parse))(input)?;
        let (input, _) = end_of_file(input)?;

        Ok((input, Computer { reg_a, reg_b, reg_c, memory }))
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
    fn test_parse_computer() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = Computer::parse(input);

        assert_eq!(
            result,
            Ok((
                "",
                Computer {
                    reg_a: 729,
                    reg_b: 0,
                    reg_c: 0,
                    memory: vec![0, 1, 5, 4, 3, 0]
                }
            ))
        );
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
