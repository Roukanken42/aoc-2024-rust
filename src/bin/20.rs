use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Tile::Wall, char('#')),
            value(Tile::Empty, char('.')),
            value(Tile::Start, char('S')),
            value(Tile::End, char('E')),
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
        let input = "S.#\n\
                     #.E";
        let result = parse(input);

        assert_eq!(
            result,
            Ok((
                "",
                vec![vec![Tile::Start, Tile::Empty, Tile::Wall], vec![Tile::Wall, Tile::Empty, Tile::End],]
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
