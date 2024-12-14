use advent_of_code::utils::location::Location;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

advent_of_code::solution!(14);

fn parse_location(input: &str) -> IResult<&str, Location<i32>> {
    map(separated_pair(i32::parse, tag(","), i32::parse), |(x, y)| Location::new(x, y))(input)
}

#[derive(Debug, PartialEq)]
struct Robot {
    position: Location<i32>,
    velocity: Location<i32>,
}

impl Parsable<'_> for Robot {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, position) = preceded(tag("p="), parse_location)(input)?;
        let (input, _) = space1(input)?;
        let (input, velocity) = preceded(tag("v="), parse_location)(input)?;

        Ok((input, Robot { position, velocity }))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    parse_input_by_lines(Robot::parse)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_inner(input, Location::new(101, 103))
}

pub fn part_one_inner(input: &str, size: Location<i32>) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_inner(input, Location::new(101, 103))
}

pub fn part_two_inner(input: &str, size: Location<i32>) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, robots) = parse(&input).unwrap();

        assert_eq!(rest, "");
        assert_eq!(
            robots[0],
            Robot {
                position: Location::new(0, 4),
                velocity: Location::new(3, -3)
            }
        );
        assert_eq!(
            robots[2],
            Robot {
                position: Location::new(10, 3),
                velocity: Location::new(-1, 2)
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(11, 7));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(11, 7));
        assert_eq!(result, None);
    }
}
