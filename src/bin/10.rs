use advent_of_code::utils::location::Access2d;
use advent_of_code::utils::{end_of_file, parse_input_by_lines};
use nom::bytes::complete::take_while;
use nom::IResult;
use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(10);

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, map) = parse_input_by_lines(take_while(|c| ('0'..='9').contains(&c)))(input)?;
    let (input, _) = end_of_file(input)?;

    let digits = map
        .iter()
        .map(|digits: &&str| digits.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    Ok((input, digits))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();

    // TODO: figure out ord on Location
    let starting_pos = map.iter_2d_keys().filter(|&loc| map.get_2d(loc) == Some(&9)).collect::<Vec<_>>();

    let mut scores = starting_pos
        .iter()
        .map(|&start| (start, HashSet::from([start])))
        .collect::<HashMap<_, _>>();
    let mut visited = HashSet::new();

    let mut queue = BinaryHeap::new();
    for &loc in &starting_pos {
        queue.push((9, loc));
    }

    let mut total_score = 0;

    while let Some((height, loc)) = queue.pop() {
        if visited.contains(&loc) {
            continue;
        }
        visited.insert(loc);

        let score = scores.get(&loc).cloned().unwrap_or_else(|| HashSet::new());

        if height == 0 {
            total_score += score.len()
        } else {
            for neighbour_loc in loc.iter_adjacent() {
                let Some(&neighbour_height) = map.get_2d(neighbour_loc) else {
                    continue;
                };

                if neighbour_height + 1 != height {
                    continue;
                }

                scores.entry(neighbour_loc).or_insert_with(|| HashSet::new()).extend(score.iter());
                queue.push((neighbour_height, neighbour_loc));
            }
        }
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse(input).unwrap();

    // TODO: figure out ord on Location
    let starting_pos = map.iter_2d_keys().filter(|&loc| map.get_2d(loc) == Some(&9)).collect::<Vec<_>>();

    let mut scores = starting_pos.iter().map(|&start| (start, 1)).collect::<HashMap<_, _>>();
    let mut visited = HashSet::new();

    let mut queue = BinaryHeap::new();
    for &loc in &starting_pos {
        queue.push((9, loc));
    }

    let mut total_score = 0;

    while let Some((height, loc)) = queue.pop() {
        if visited.contains(&loc) {
            continue;
        }
        visited.insert(loc);

        let score = scores[&loc];

        if height == 0 {
            total_score += score
        } else {
            for neighbour_loc in loc.iter_adjacent() {
                let Some(&neighbour_height) = map.get_2d(neighbour_loc) else {
                    continue;
                };

                if neighbour_height + 1 != height {
                    continue;
                }

                let neighbour_score = scores.get(&neighbour_loc).copied().unwrap_or(0);
                scores.insert(neighbour_loc, neighbour_score + score);
                queue.push((neighbour_height, neighbour_loc));
            }
        }
    }

    Some(total_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result[0], vec![8, 9, 0, 1, 0, 1, 2, 3]);
        assert_eq!(result[1], vec![7, 8, 1, 2, 1, 8, 7, 4]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
