use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::character::complete::space1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(1);

pub fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let parse_line = separated_pair(u64::parse, space1, u64::parse);
    parse_input_by_lines(parse_line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
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
        let result = parse(&input);
        assert_eq!(result, Ok(("", vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)])));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
