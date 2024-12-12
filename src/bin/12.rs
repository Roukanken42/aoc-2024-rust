use advent_of_code::utils::location::direction::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::parse_input_by_lines;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;
use std::collections::HashSet;
use std::iter::successors;

advent_of_code::solution!(12);

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let line = map(alpha1, |line: &str| line.chars().collect());
    parse_input_by_lines(line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, map) = parse(input).unwrap();

    let mut visited = HashSet::new();

    fn scout(loc: Location<i32>, category: &char, map: &Vec<Vec<char>>, visited: &mut HashSet<Location<i32>>) -> (u32, u32) {
        let current = map.get_2d(loc);

        if current != Some(category) {
            (0, 1)
        } else if visited.contains(&loc) {
            (0, 0)
        } else {
            visited.insert(loc);

            let (area, perimeter) = loc
                .iter_adjacent()
                .into_iter()
                .map(|loc| scout(loc, category, map, visited))
                .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
                .unwrap_or((0, 0));

            (area + 1, perimeter)
        }
    }

    let mut price = 0;

    for loc in map.iter_2d_keys() {
        if visited.contains(&loc) {
            continue;
        }

        let (area, perimeter) = scout(loc, &map.get_2d(loc).unwrap(), &map, &mut visited);
        price += area * perimeter;
    }

    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse(input).unwrap();

    let mut visited = HashSet::new();

    fn area(loc: Location<i32>, category: &char, map: &Vec<Vec<char>>, visited: &mut HashSet<Location<i32>>) -> u32 {
        if visited.contains(&loc) {
            0
        } else if map.get_2d(loc) != Some(category) {
            0
        } else {
            visited.insert(loc);

            loc.iter_adjacent().into_iter().map(|loc| area(loc, category, map, visited)).sum::<u32>() + 1
        }
    }

    fn find_all_side_parts(location: Location<i32>, category: &char, map: &Vec<Vec<char>>) -> Vec<(Location<i32>, Location<i32>)> {
        let mut visited = HashSet::new();
        let mut queue = vec![location];

        let mut sides = vec![];

        while let Some(location) = queue.pop() {
            if visited.contains(&location) {
                continue;
            }
            visited.insert(location);

            for &direction in [UP, RIGHT, DOWN, LEFT].iter() {
                let loc = location + direction;

                if map.get_2d(loc) == Some(category) {
                    queue.push(loc);
                } else {
                    sides.push((location, direction.rotate_90_ccw()));
                }
            }
        }

        sides
    }

    fn find_next_corner(
        corner: (Location<i32>, Location<i32>),
        category: &char,
        map: &Vec<Vec<char>>,
        visited: &mut HashSet<(Location<i32>, Location<i32>)>,
    ) -> Option<(Location<i32>, Location<i32>)> {
        let (location, forward) = corner;
        let right = forward.rotate_90_cw();

        let mut current = location;
        while !visited.contains(&(current, forward))
            && map.get_2d(current + right) != Some(category)
            && map.get_2d(current + forward) == Some(category)
        {
            visited.insert((current, forward));
            current = current + forward;
        }

        if visited.contains(&(current, forward)) {
            None
        } else {
            visited.insert((current, forward));

            if map.get_2d(current + right) == Some(category) {
                Some((current + right, right))
            } else {
                Some((current, forward.rotate_90_ccw()))
            }
        }
    }

    fn corners(loc: Location<i32>, category: &char, map: &Vec<Vec<char>>) -> u32 {
        let side_parts = find_all_side_parts(loc, category, map);

        let mut visited_corners = HashSet::new();
        let mut corners = 0;

        for (location, direction) in side_parts {
            corners += successors(find_next_corner((location, direction), category, map, &mut visited_corners), |corner| {
                find_next_corner(*corner, category, map, &mut visited_corners)
            })
            .count();
        }

        corners as u32
    }

    let mut price = 0;

    for loc in map.iter_2d_keys() {
        if visited.contains(&loc) {
            continue;
        }

        let category = map.get_2d(loc).unwrap();
        let area = area(loc, &category, &map, &mut visited);
        let corners = corners(loc, &category, &map);

        price += area * corners;
    }

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result.len(), 10);
        assert!(result.iter().all(|row| row.len() == 10));

        assert_eq!(result[0], vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F']);
        assert_eq!(result[2], vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F']);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
