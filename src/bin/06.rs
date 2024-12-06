use advent_of_code::utils::location::{direction, Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;
use num::Zero;
use std::collections::HashSet;

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

struct Map {
    obstacles: HashSet<Location<i32>>,
    start: Location<i32>,
    size: Location<i32>,
}

impl Map {
    fn new(tiles: &Vec<Vec<Tile>>) -> Self {
        let obstacles = tiles.iter_2d_keys().filter(|&loc| tiles.get_2d(loc) == Some(&Tile::Obstacle)).collect();
        let guard = tiles.iter_2d_keys().find(|&loc| tiles.get_2d(loc) == Some(&Tile::Guard)).unwrap();
        let size = Location::new(tiles[0].len() as i32, tiles.len() as i32);

        Self {
            obstacles,
            start: guard,
            size,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let map = Map::new(&input);

    let mut visited = HashSet::new();
    let mut current = map.start;
    let mut direction = direction::UP;

    while (Location::zero()..map.size).contains(&current) {
        if current.x > map.size.x || current.y > map.size.y {
            panic!("Out of bounds");
        }

        visited.insert(current);

        while map.obstacles.contains(&(current + direction)) {
            direction = direction.rotate_90_cw();
        }

        current = current + direction;
    }

    Some(visited.len())
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
    fn test_map_new() {
        let map = Map::new(&vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Obstacle, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Guard, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        ]);

        assert_eq!(map.obstacles, HashSet::from([Location::new(3, 1)]));
        assert_eq!(map.start, Location::new(2, 3));
        assert_eq!(map.size, Location::new(5, 5));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
