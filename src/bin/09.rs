use advent_of_code::utils::end_of_file;
use itertools::Itertools;
use nom::bytes::complete::take_while;
use nom::IResult;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(9);

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = take_while(|c| ('0'..='9').contains(&c))(input)?;
    let (input, _) = end_of_file(input)?;

    let digits = digits.chars().map(|c| c.to_digit(10).unwrap()).collect();
    Ok((input, digits))
}

fn contribute_checksum(at_block: u32, size: u32) -> u32 {
    size * (at_block * 2 + size - 1) / 2
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let last_id = input.len() / 2;

    let mut last_iter = input.iter().rev().copied().tuples().enumerate().map(|(i, (a, b))| (last_id - i, (a, b)));

    let mut back_id;
    let mut back_filled;
    let mut current_block = 0u32;

    (back_id, (back_filled, _)) = last_iter.next().unwrap();

    let mut checksum = 0usize;
    for (current_id, (&filled, &free)) in input.iter().tuples().enumerate() {
        if current_id == back_id {
            checksum += contribute_checksum(current_block, back_filled) as usize * back_id;
            break;
        }

        checksum += contribute_checksum(current_block, filled) as usize * current_id;
        current_block += filled;

        let mut free = free;
        while free > 0 && back_id != current_id {
            let taking = free.min(back_filled);
            back_filled -= taking;
            free -= taking;

            checksum += contribute_checksum(current_block, taking) as usize * back_id;
            current_block += taking;

            if back_filled == 0 {
                (back_id, (back_filled, _)) = last_iter.next().unwrap();
            }
        }
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let mut free_spaces = (1..=9).map(|i| (i, BinaryHeap::new())).collect::<HashMap<_, _>>();

    let mut current_block = 0u32;
    for (filled, free) in input.iter().tuples() {
        if free > &0 {
            free_spaces.get_mut(free).unwrap().push(Reverse(current_block + filled));
        }
        current_block += filled + free;
    }

    current_block += input.last().unwrap();

    let mut checksum = 0usize;

    for (i, (&filled, &free)) in input.iter().rev().tuples().enumerate() {
        let id = input.len() / 2 - i;
        current_block -= filled;

        let space_size = (filled..=9)
            .filter(|size| free_spaces.get(size).unwrap().peek().unwrap_or(&Reverse(0)) > &Reverse(current_block))
            .max_by_key(|size| free_spaces.get(size).unwrap().peek().unwrap_or(&Reverse(u32::MAX)))
            .unwrap_or(0);

        let tree = free_spaces.get_mut(&space_size);
        if tree.is_none_or(|tree| tree.is_empty()) {
            checksum += contribute_checksum(current_block, filled) as usize * id;
        } else {
            let Reverse(block) = free_spaces.get_mut(&space_size).unwrap().pop().unwrap();
            checksum += contribute_checksum(block, filled) as usize * id;

            if space_size > filled {
                free_spaces.get_mut(&(space_size - filled)).unwrap().push(Reverse(block + filled));
            }
        }

        current_block -= free;
    }

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result, vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]);
    }

    #[test]
    fn test_contribute_checksum() {
        assert_eq!(contribute_checksum(0, 1), 0);
        assert_eq!(contribute_checksum(1, 1), 1);
        assert_eq!(contribute_checksum(0, 2), 1);
        assert_eq!(contribute_checksum(1, 2), 3);
        assert_eq!(contribute_checksum(2, 2), 5);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
