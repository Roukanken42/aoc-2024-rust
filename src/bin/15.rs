use crate::Tile::{Box, Empty, Wall};
use crate::Tile2::{BoxLeft, BoxRight};
use advent_of_code::utils::location::direction::{DOWN, LEFT, RIGHT, UP, ZERO};
use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::{end_of_file, Parsable};
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::{opt, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;
use std::collections::HashMap;
use Tile::Robot;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile2 {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Empty, char('.')),
            value(Wall, char('#')),
            value(Box, char('O')),
            value(Robot, char('@')),
        ))(input)
    }
}

struct Input {
    map: Vec<Vec<Tile>>,
    robot: Location<i32>,
    moves: Vec<Location<i32>>,
}

impl Input {
    fn scale_up_map(self: &Self) -> Vec<Vec<Tile2>> {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|tile| match tile {
                        Empty => [Tile2::Empty, Tile2::Empty],
                        Wall => [Tile2::Wall, Tile2::Wall],
                        Box => [Tile2::BoxLeft, Tile2::BoxRight],
                        Robot => [Tile2::Empty, Tile2::Empty],
                    })
                    .collect()
            })
            .collect()
    }
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

        let robot = map.iter_2d_keys().find(|&loc| map.get_2d(loc) == Some(&Robot)).unwrap();
        map.set_2d(robot, Empty);

        let (input, _) = end_of_file(input)?;

        Ok((input, Input { map, robot, moves }))
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, input) = Input::parse(input).unwrap();

    let mut map = input.map;
    let mut robot = input.robot;

    for dir in input.moves {
        let new_robot = robot + dir;

        match map.get_2d(new_robot) {
            Some(Empty) => robot = new_robot,
            Some(Box) => {
                let first_non_box = new_robot.iter_ray(dir).find(|loc| map.get_2d(*loc) != Some(&Box)).unwrap();
                if let Some(Empty) = map.get_2d(first_non_box) {
                    map.set_2d(first_non_box, Box);
                    map.set_2d(new_robot, Empty);
                    robot = new_robot;
                }
            }
            _ => { /* no op */ }
        }
    }

    Some(
        map.iter_2d_keys()
            .filter(|&loc| map.get_2d(loc) == Some(&Box))
            .map(|loc| loc.x + loc.y * 100)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<i32> {
    let (_, input) = Input::parse(_input).unwrap();

    let mut map = input.scale_up_map();
    let mut robot = Location::new(input.robot.x * 2, input.robot.y);

    fn can_move_to(map: &Vec<Vec<Tile2>>, loc: Location<i32>, dir: Location<i32>, moving: &mut HashMap<Location<i32>, bool>) -> bool {
        if let Some(res) = moving.get(&loc) {
            return *res;
        }

        match map.get_2d(loc) {
            Some(Tile2::Empty) => true,
            Some(tile) if tile == &BoxLeft || tile == &BoxRight => {
                let left = loc + if tile == &BoxLeft { ZERO } else { LEFT };
                let right = loc + if tile == &BoxRight { ZERO } else { RIGHT };

                let can_move = if dir.x == 0 {
                    let can_move_left = can_move_to(map, left + dir, dir, moving);
                    let can_move_right = can_move_to(map, right + dir, dir, moving);

                    can_move_left && can_move_right
                } else {
                    let to_move = if dir.x < 0 { left } else { right };
                    can_move_to(map, to_move + dir, dir, moving)
                };

                moving.insert(left, can_move);
                moving.insert(right, can_move);

                can_move
            }
            _ => false,
        }
    }

    for dir in input.moves {
        let new_robot = robot + dir;

        let moving = &mut HashMap::new();
        let can_move = can_move_to(&map, new_robot, dir, moving);

        if can_move {
            robot = new_robot;

            let to_update = moving
                .iter()
                .map(|(&loc, _)| (loc + dir, map.get_2d(loc).copied().unwrap()))
                .collect::<HashMap<_, _>>();

            moving.iter().filter(|(&loc, _)| !to_update.contains_key(&(loc))).for_each(|(&loc, _)| {
                map.set_2d(loc, Tile2::Empty);
            });

            for (new_loc, tile) in to_update {
                map.set_2d(new_loc, tile);
            }
        }
    }

    Some(
        map.iter_2d_keys()
            .filter(|&loc| map.get_2d(loc) == Some(&BoxLeft))
            .map(|loc| loc.x + loc.y * 100)
            .sum(),
    )
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
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_large() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9021));
    }
}
