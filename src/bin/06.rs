use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        alt((
            value(Tile::Empty, tag(".")),
            value(Tile::Obstacle, tag("#")),
            value(Tile::Guard, tag("^")),
        ))(input)
    }
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<Tile>>> {
    parse_input_by_lines(many1(Tile::parse))(input)
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
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);

        let (str, input) = result.unwrap();

        assert!(str.is_empty());
        assert_eq!(
            input[0][4],
            Tile::Obstacle,
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
