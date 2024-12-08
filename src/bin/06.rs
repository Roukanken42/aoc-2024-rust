use advent_of_code::utils::location::direction::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::location::{direction, Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;
use num::Zero;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

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
    range: RangeInclusive<Location<i32>>,
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
            range: Location::zero().square_range(size),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let map = Map::new(&input);

    let mut visited = HashSet::new();
    let mut current = map.start;
    let mut direction = direction::UP;

    while map.range.contains(&current) {
        visited.insert(current);

        while map.obstacles.contains(&(current + direction)) {
            direction = direction.rotate_90_cw();
        }

        current = current + direction;
    }

    Some(visited.len())
}

fn sim_obstacle_in_front(
    map: &Map,
    start: Location<i32>,
    start_direction: Location<i32>,
    visited_states: &HashSet<(Location<i32>, Location<i32>)>,
    shortcut_map: &HashMap<(Location<i32>, Location<i32>), (Location<i32>, Location<i32>)>,
) -> bool {
    let extra_obstacle = start + start_direction;

    let mut current = start;
    let mut direction = start_direction.rotate_90_cw();
    let mut local_visited_states = HashSet::new();

    while map.range.contains(&current) {
        if visited_states.contains(&(current, direction)) || local_visited_states.contains(&(current, direction)) {
            return true;
        }

        local_visited_states.insert((current, direction));

        if current.x != extra_obstacle.x && current.y != extra_obstacle.y {
            (current, direction) = shortcut_map.get(&(current, direction)).copied().unwrap();
        } else {
            while map.obstacles.contains(&(current + direction)) || current + direction == extra_obstacle {
                direction = direction.rotate_90_cw();
                local_visited_states.insert((current, direction));
            }

            current = current + direction;
        }
    }

    false
}

fn construct_shortcut_map(map: &Map) -> HashMap<(Location<i32>, Location<i32>), (Location<i32>, Location<i32>)> {
    let mut result = HashMap::new();

    for direction in [UP, RIGHT, DOWN, LEFT] {
        let facing = -direction;

        let mut start = Location::new(
            if direction.x < 0 { map.size.x - 1 } else { 0 },
            if direction.y < 0 { map.size.y - 1 } else { 0 },
        );
        let delta = Location::new(if direction.x == 0 { 1 } else { 0 }, if direction.y == 0 { 1 } else { 0 });

        while map.range.contains(&start) {
            let mut current = start;
            let mut last_obstacle_hit = start + facing;

            while map.range.contains(&current) {
                if map.obstacles.contains(&current) {
                    last_obstacle_hit = current + direction;
                } else {
                    result.insert((current, facing), (last_obstacle_hit, facing.rotate_90_cw()));
                }

                current = current + direction;
            }

            start = start + delta;
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let map = Map::new(&input);
    let shortcut_map = construct_shortcut_map(&map);

    let mut visited_fields = HashSet::new();
    let mut visited_states = HashSet::new();
    let mut current = map.start;
    let mut direction = direction::UP;
    let mut extra_obstacle_positions = HashSet::new();

    while map.range.contains(&current) {
        visited_fields.insert(current);
        visited_states.insert((current, direction));

        while map.obstacles.contains(&(current + direction)) {
            direction = direction.rotate_90_cw();
            visited_states.insert((current, direction));
        }

        if map.range.contains(&(current + direction)) && !visited_fields.contains(&(current + direction)) {
            let obstacle_causes_loop = sim_obstacle_in_front(&map, current, direction, &visited_states, &shortcut_map);
            if obstacle_causes_loop {
                extra_obstacle_positions.insert(current + direction);
            }
        }

        current = current + direction;
    }

    Some(extra_obstacle_positions.len())
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
        assert_eq!(input[0][4], Tile::Obstacle,);
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
    fn test_construct_shortcut_map() {
        let map = Map::new(&vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Obstacle, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Guard, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        ]);

        let shortcut_map = construct_shortcut_map(&map);

        assert_eq!(shortcut_map.get(&(Location::new(3, 0), UP)), Some(&(Location::new(3, -1), RIGHT)));
        assert_eq!(shortcut_map.get(&(Location::new(3, 4), UP)), Some(&(Location::new(3, 2), RIGHT)));
        assert_eq!(shortcut_map.get(&(Location::new(4, 4), UP)), Some(&(Location::new(4, -1), RIGHT)));
        assert_eq!(shortcut_map.get(&(Location::new(1, 1), RIGHT)), Some(&(Location::new(2, 1), DOWN)));
        assert_eq!(shortcut_map.get(&(Location::new(4, 1), RIGHT)), Some(&(Location::new(5, 1), DOWN)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
