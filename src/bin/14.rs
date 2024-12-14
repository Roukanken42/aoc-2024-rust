use advent_of_code::utils::location::Location;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use num::traits::Euclid;
use std::cmp::Ordering::Equal;

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

impl Robot {
    fn simulate_pos(&self, time: i32, area_size: &Location<i32>) -> Location<i32> {
        (self.position + self.velocity * time).rem_euclid(area_size)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    parse_input_by_lines(Robot::parse)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, Location::new(101, 103))
}

pub fn part_one_inner(input: &str, size: Location<i32>) -> Option<usize> {
    let (_, robots) = parse(input).unwrap();

    let res = robots
        .into_iter()
        .map(|robot| robot.simulate_pos(100, &size))
        .map(|pos| (pos.x.cmp(&(size.x / 2)), pos.y.cmp(&(size.y / 2))))
        .filter(|(x, y)| x != &Equal && y != &Equal)
        .counts()
        .iter()
        .fold(1, |acc, (_, &count)| acc * count);

    Some(res)
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
    fn test_simulate_pos() {
        let robot = Robot {
            position: Location::new(2, 4),
            velocity: Location::new(2, -3),
        };

        let area_size = Location::new(11, 7);

        assert_eq!(robot.simulate_pos(1, &area_size), Location::new(4, 1));
        assert_eq!(robot.simulate_pos(2, &area_size), Location::new(6, 5));
        assert_eq!(robot.simulate_pos(3, &area_size), Location::new(8, 2));
        assert_eq!(robot.simulate_pos(4, &area_size), Location::new(10, 6));
        assert_eq!(robot.simulate_pos(5, &area_size), Location::new(1, 3));
    }

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(11, 7));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(11, 7));
        assert_eq!(result, None);
    }
}
