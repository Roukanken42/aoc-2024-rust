use crate::Tile::{Empty, End, Start, Wall};
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Empty, char('.')),
            value(Wall, char('#')),
            value(Start, char('S')),
            value(End, char('E')),
        ))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    parse_input_by_lines(many1(Tile::parse))(input)
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
        let input = advent_of_code::template::read_file_part("examples", DAY, 1);
        let (rest, input) = parse(&input).unwrap();

        assert_eq!(rest, "");

        assert_eq!((input[0].len(), input.len()), (15, 15));
        assert_eq!(
            input[2],
            vec![Wall, Empty, Wall, Empty, Wall, Wall, Wall, Empty, Wall, Empty, Wall, Wall, Wall, Empty, Wall]
        );
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
