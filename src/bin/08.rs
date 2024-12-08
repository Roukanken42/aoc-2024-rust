use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::parse_input;
use itertools::Itertools;
use nom::character::complete::{anychar, line_ending};
use nom::combinator::map;
use nom::multi::{many1, many_till};
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<char>>> {
    let parse_line = map(many_till(anychar, line_ending), |x| x.0);
    parse_input(many1(parse_line))(input)
}

#[derive(Debug, PartialEq)]
struct Map {
    antennas: HashMap<char, Vec<Location<i32>>>,
    size: Location<i32>,
}

impl Map {
    fn new(tiles: &Vec<Vec<char>>) -> Self {
        let antennas = tiles
            .iter_2d_keys()
            .map(|loc| (tiles.get_2d(loc).copied().unwrap(), loc))
            .filter(|(c, _)| *c != '.')
            .into_group_map()
            .into_iter()
            .collect();

        let size = Location::new(tiles[0].len() as i32, tiles.len() as i32);

        Self { antennas, size }
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
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result[1][8], '0');
        assert_eq!(result[5][6], 'A');
    }

    #[test]
    fn test_map_mew() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (_, result) = parse(&input).unwrap();
        let map = Map::new(&result);

        assert_eq!(
            map,
            Map {
                antennas: HashMap::from([
                    ('A', vec![Location::new(6, 5), Location::new(8, 8), Location::new(9, 9)]),
                    (
                        '0',
                        vec![Location::new(8, 1), Location::new(5, 2), Location::new(7, 3), Location::new(4, 4)]
                    ),
                ]),
                size: Location::new(12, 12),
            }
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
