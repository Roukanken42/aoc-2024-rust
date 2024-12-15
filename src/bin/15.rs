use crate::Tile::{Box, Empty, Wall};
use advent_of_code::utils::location::direction::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::{end_of_file, Parsable};
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{opt, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Tile::Empty, char('.')),
            value(Tile::Wall, char('#')),
            value(Tile::Box, char('O')),
            value(Tile::Robot, char('@')),
        ))(input)
    }
}

struct Input {
    map: Vec<Vec<Tile>>,
    robot: Location<i32>,
    moves: Vec<Location<i32>>,
}

fn parse_direction(input: &str) -> IResult<&str, Location<i32>> {
    alt((
        value(UP, char('^')),
        value(RIGHT, char('>')),
        value(DOWN, char('v')),
        value(LEFT, char('<')),
    ))(input)
}

impl Parsable<'_> for Input {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, mut map) = separated_list1(line_ending, many1(Tile::parse))(input)?;
        let (input, _) = many1(line_ending)(input)?;
        let (input, moves) = many1(preceded(opt(line_ending), parse_direction))(input)?;

        let robot = map.iter_2d_keys().find(|&loc| map.get_2d(loc) == Some(&Tile::Robot)).unwrap();
        map.set_2d(robot, Empty);

        let (input, _) = end_of_file(input)?;

        Ok((input, Input { map, robot, moves }))
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let (rest, input) = Input::parse(&input).unwrap();

        assert_eq!(rest, "");

        assert_eq!((input.map[0].len(), input.map.len()), (10, 10));
        assert_eq!(input.map[4], vec![Wall, Empty, Empty, Box, Empty, Empty, Empty, Box, Empty, Wall]);

        assert_eq!(input.robot, Location::new(4, 4));

        assert_eq!(input.moves.len(), 700);
        assert_eq!(input.moves[71], DOWN);
    }

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_large() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, None);
    }
}
