use advent_of_code::utils::location::Location;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use hashbrown::HashSet;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;
use num::Zero;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(18);

fn parse(input: &str) -> IResult<&str, Vec<Location<i32>>> {
    let parse_location = map(separated_pair(i32::parse, char(','), i32::parse), |(x, y)| Location::new(x, y));
    parse_input_by_lines(parse_location)(input)
}

pub fn part_one(input: &str) -> Option<i32> {
    part_one_inner(input, Location::new(70, 70), 1024)
}

fn part_one_inner(input: &str, size: Location<i32>, simulate: usize) -> Option<i32> {
    let (_, bytes) = parse(input).unwrap();
    let map = bytes.iter().take(simulate).cloned().collect::<HashSet<_>>();
    let range = Location::zero().square_range(size + Location::new(1, 1));

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Reverse((size.manhattan_distance(Location::zero()), 0, Location::new(0, 0))));

    while let Some(Reverse((_, steps, loc))) = queue.pop() {
        if visited.contains(&loc) {
            continue;
        }
        visited.insert(loc);

        if loc == size {
            return Some(steps);
        }

        for neighbour_loc in loc.iter_adjacent() {
            if map.contains(&neighbour_loc) || !range.contains(&neighbour_loc) {
                continue;
            }

            queue.push(Reverse((size.manhattan_distance(neighbour_loc) + steps + 1, steps + 1, neighbour_loc)));
        }
    }

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
        let (rest, bytes) = parse(&input).unwrap();

        assert_eq!(rest, "");
        assert_eq!(bytes.len(), 25);
        assert_eq!(bytes[0], Location::new(5, 4));
        assert_eq!(bytes[1], Location::new(4, 2));
    }

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(6, 6), 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
