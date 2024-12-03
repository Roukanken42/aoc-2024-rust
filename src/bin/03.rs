use advent_of_code::utils::Parsable;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::multi::{many0, many1, many_till};
use nom::sequence::{terminated, tuple};
use nom::IResult;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, left, _, right, _)) =
        tuple((tag("mul("), u32::parse, tag(","), u32::parse, tag(")")))(input)?;
    Ok((input, Instruction::Mul(left, right)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

impl Parsable<'_> for Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((parse_do, parse_dont, parse_mul))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(
        many1(map(many_till(anychar, Instruction::parse), |(_, instr)| instr)),
        many0(anychar),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = parse(input).unwrap();
    Some(input.iter().map(|instr| match instr {
        Instruction::Mul(a, b) => a * b,
        _ => 0
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, input) = parse(input).unwrap();

    let mut enabled = true;
    Some(input.iter().map(|instr| match instr {
        Instruction::Mul(a, b) => if enabled { a * b } else { 0 },
        Instruction::Do => {
            enabled = true;
            0
        }
        Instruction::Dont => {
            enabled = false;
            0
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction::{Do, Dont};
    use Instruction::Mul;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);
        assert_eq!(result, Ok(("", vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)])));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
