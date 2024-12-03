use advent_of_code::utils::Parsable;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::multi::{many0, many1, many_till};
use nom::sequence::{terminated, tuple};
use nom::IResult;

advent_of_code::solution!(3);

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (_, left, _, right, _)) =
        tuple((tag("mul("), u32::parse, tag(","), u32::parse, tag(")")))(input)?;
    Ok((input, (left, right)))
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    terminated(
        many1(map(many_till(anychar, parse_mul), |(_, mul)| mul)),
        many0(anychar),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = parse(input).unwrap();
    Some(input.iter().map(|(l, r)| l * r).sum())
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
        assert_eq!(result, Ok(("", vec![(2, 4), (5, 5), (11, 8), (8, 5)])));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
