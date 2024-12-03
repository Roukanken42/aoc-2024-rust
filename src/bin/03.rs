use advent_of_code::utils::{parse_input, Parsable};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{map, value};
use nom::multi::{many0, many1, many_till};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use Instruction::{Do, Dont, Mul};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Parsable<'_> for Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let recognize_mul = delimited(
            tag("mul("),
            separated_pair(u32::parse, tag(","), u32::parse),
            tag(")"),
        );

        alt((
            value(Do, tag("do()")),
            value(Dont, tag("don't()")),
            map(recognize_mul, |(a, b)| Mul(a, b)),
        ))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    parse_input(terminated(
        many1(map(many_till(anychar, Instruction::parse), |t| t.1)),
        many0(anychar),
    ))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = parse(input).unwrap();

    Some(
        input
            .iter()
            .map(|instr| match instr {
                Mul(a, b) => a * b,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, input) = parse(input).unwrap();

    let mut enabled = true;
    Some(
        input
            .iter()
            .map(|instr| {
                match instr {
                    Do => enabled = true,
                    Dont => enabled = false,
                    _ => (),
                }

                match instr {
                    Mul(a, b) if enabled => a * b,
                    _ => 0
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]
            ))
        );
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
