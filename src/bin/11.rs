use advent_of_code::utils::{parse_input, Parsable};
use nom::IResult;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    parse_input(Vec::parse)(input)
}

fn next(stone: u64, step: u64) -> u64 {
    if step == 0 {
        return 1;
    }
    if stone == 0 {
        return next(1, step - 1);
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let half = 10u64.pow(digits / 2);
        next(stone / half, step - 1) + next(stone % half, step - 1)
    } else {
        next(stone * 2024, step - 1)
    }
}

fn next_memo(stone: u64, step: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if step == 0 {
        return 1;
    }
    if let Some(result) = memo.get(&(stone, step)) {
        return *result;
    }

    let result;
    if stone == 0 {
        result = next_memo(1, step - 1, memo);
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let half = 10u64.pow(digits / 2);
            result = next_memo(stone / half, step - 1, memo) + next_memo(stone % half, step - 1, memo);
        } else {
            result = next_memo(stone * 2024, step - 1, memo);
        }
    }

    memo.insert((stone, step), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, input) = parse(input).unwrap();

    Some(input.iter().map(|&stone| next(stone, 25)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, input) = parse(input).unwrap();

    let memo = &mut HashMap::new();
    Some(input.iter().map(|&stone| next_memo(stone, 75, memo)).sum())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result, vec![125, 17]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
