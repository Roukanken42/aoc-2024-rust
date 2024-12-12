use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::parse_input_by_lines;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;
use std::collections::HashSet;

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

pub fn part_two(_input: &str) -> Option<u32> {
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
        assert_eq!(result, None);
    }
}
