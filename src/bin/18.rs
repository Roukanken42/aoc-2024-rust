use advent_of_code::utils::location::Location;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use hashbrown::{HashMap, HashSet};
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;
use num::Zero;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

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

pub fn part_two(input: &str) -> Option<String> {
    part_two_inner(input, Location::new(70, 70))
}

struct UnionFind<T: Eq + Hash> {
    parents: HashMap<T, T>,
}

impl<T: Eq + Hash + Copy> UnionFind<T> {
    fn new() -> Self {
        Self { parents: HashMap::new() }
    }

    fn add(&mut self, item: T) {
        self.parents.insert(item, item);
    }

    fn contains(&self, item: T) -> bool {
        self.parents.contains_key(&item)
    }

    fn find(&mut self, item: T) -> T {
        let parent = *self.parents.get(&item).unwrap_or(&item);

        if parent == item {
            parent
        } else {
            let root = self.find(parent);
            self.parents.insert(item, root);
            root
        }
    }

    fn union(&mut self, a: T, b: T) {
        let a_root = self.find(a);
        let b_root = self.find(b);

        self.parents.insert(a_root, b_root);
    }
}

fn part_two_inner(input: &str, size: Location<i32>) -> Option<String> {
    let (_, bytes) = parse(input).unwrap();
    let mut map = bytes.iter().cloned().collect::<HashSet<_>>();

    let mut union_find = UnionFind::new();

    let range = Location::zero().square_range(size + Location::new(1, 1));

    let mut queue = Vec::new();

    for loc in Location::zero().iter_range(size + Location::new(1, 1)) {
        if map.contains(&loc) {
            continue;
        }

        queue.push((loc, loc));
    }

    while let Some((loc, parent)) = queue.pop() {
        if union_find.contains(loc) {
            continue;
        }

        union_find.add(loc);
        union_find.union(loc, parent);

        for neighbour_loc in loc.iter_adjacent() {
            if !range.contains(&neighbour_loc) || map.contains(&neighbour_loc) {
                continue;
            }

            queue.push((neighbour_loc, parent));
        }
    }

    for &loc in bytes.iter().rev() {
        map.remove(&loc);

        union_find.add(loc);

        for neighbour_loc in loc.iter_adjacent() {
            if !range.contains(&neighbour_loc) || map.contains(&neighbour_loc) {
                continue;
            }

            union_find.union(neighbour_loc, loc);
        }

        if union_find.find(Location::zero()) == union_find.find(size) {
            return Some(format!("{},{}", loc.x, loc.y));
        }
    }

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
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(6, 6));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
