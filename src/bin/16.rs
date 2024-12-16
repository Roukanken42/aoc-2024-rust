use crate::Tile::{Empty, End, Start, Wall};
use advent_of_code::utils::location::direction::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::location::Access2d;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    let (_, map) = parse(input).unwrap();

    let start = map.iter_2d_keys().find(|&loc| map.get_2d(loc) == Some(&Start)).unwrap();

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start, RIGHT)));

    let mut visited = HashMap::new();

    // TODO: A* ?
    while let Some(Reverse((steps, loc, dir))) = queue.pop() {
        if visited.contains_key(&(loc, dir)) {
            continue;
        }
        visited.insert((loc, dir), steps);

        if let Some(&End) = map.get_2d(loc) {
            return Some(steps);
        }

        let new_loc = loc + dir;
        if map.get_2d(new_loc) != Some(&Wall) {
            queue.push(Reverse((steps + 1, new_loc, dir)));
        }

        for &new_dir in [dir.rotate_90_cw(), dir.rotate_90_ccw()].iter() {
            queue.push(Reverse((steps + 1_000, loc, new_dir)));
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();

    let start = map.iter_2d_keys().find(|&loc| map.get_2d(loc) == Some(&Start)).unwrap();
    let end = map.iter_2d_keys().find(|&loc| map.get_2d(loc) == Some(&End)).unwrap();

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start, RIGHT, (start, RIGHT))));

    let mut visited: HashMap<_, (i32, HashSet<_>)> = HashMap::new();
    let mut found_end = false;

    while let Some(Reverse((steps, loc, dir, last_tile))) = queue.pop() {
        if let Some((min_steps, last_tiles)) = visited.get_mut(&(loc, dir)) {
            if min_steps == &steps {
                last_tiles.insert(last_tile);
            }
            continue;
        }
        visited.entry((loc, dir)).or_insert_with(|| (steps, HashSet::new())).1.insert(last_tile);

        if map.get_2d(loc) == Some(&End) {
            found_end = true;
            continue;
        }

        if found_end {
            continue;
        }

        let new_loc = loc + dir;
        if map.get_2d(new_loc) != Some(&Wall) {
            queue.push(Reverse((steps + 1, new_loc, dir, (loc, dir))));
        }

        for &new_dir in [dir.rotate_90_cw(), dir.rotate_90_ccw()].iter() {
            queue.push(Reverse((steps + 1_000, loc, new_dir, (loc, dir))));
        }
    }

    let mut best_paths = HashSet::new();
    let mut queue = vec![(end, UP), (end, RIGHT), (end, DOWN), (end, LEFT)];

    while let Some((loc, dir)) = queue.pop() {
        if best_paths.contains(&(loc, dir)) {
            continue;
        }
        best_paths.insert((loc, dir));

        if let Some(last_tile) = visited.get(&(loc, dir)) {
            queue.extend(last_tile.1.iter());
        }
    }

    Some(best_paths.iter().map(|&(loc, _)| loc).unique().count())
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_large() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_large() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
