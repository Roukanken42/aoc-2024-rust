use advent_of_code::utils::location::Location;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending};
use nom::sequence::delimited;
use nom::IResult;

advent_of_code::solution!(13);

#[derive(Debug, PartialEq)]
struct Machine {
    button_a: Location<i64>,
    button_b: Location<i64>,
    prize: Location<i64>,
}

impl Parsable<'_> for Machine {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, button_a) = delimited(tag("Button A: "), parse_location('+'), line_ending)(input)?;
        let (input, button_b) = delimited(tag("Button B: "), parse_location('+'), line_ending)(input)?;
        let (input, prize) = delimited(tag("Prize: "), parse_location('='), line_ending)(input)?;

        Ok((input, Machine { button_a, button_b, prize }))
    }
}

fn parse_location<'a>(sep: char) -> impl FnMut(&'a str) -> IResult<&'a str, Location<i64>> {
    move |input: &str| {
        let (input, _) = tag("X")(input)?;
        let (input, _) = char(sep)(input)?;
        let (input, x) = i64::parse(input)?;

        let (input, _) = tag(", ")(input)?;

        let (input, _) = tag("Y")(input)?;
        let (input, _) = char(sep)(input)?;
        let (input, y) = i64::parse(input)?;

        Ok((input, Location::new(x, y)))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    parse_input_by_lines(Machine::parse)(input)
}

fn solve(a: Location<i64>, b: Location<i64>, p: Location<i64>) -> Option<i64> {
    let b_count = (a.y * p.x - a.x * p.y) / (b.x * a.y - b.y * a.x);
    let a_count = (p.x - b.x * b_count) / a.x;

    if (a * a_count + b * b_count) != p {
        None
    } else {
        Some(a_count * 3 + b_count)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, machines) = parse(input).unwrap();

    Some(
        machines
            .iter()
            .flat_map(|machine| solve(machine.button_a, machine.button_b, machine.prize))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, machines) = parse(input).unwrap();

    Some(
        machines
            .iter()
            .flat_map(|machine| {
                solve(
                    machine.button_a,
                    machine.button_b,
                    machine.prize + Location::new(10000000000000, 10000000000000),
                )
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Parser;

    #[test]
    fn test_parse_location() {
        assert_eq!(parse_location('=').parse("X=1, Y=2"), Ok(("", Location::new(1, 2))));
        assert_eq!(parse_location('+').parse("X+42, Y+47"), Ok(("", Location::new(42, 47))));
    }

    #[test]
    fn test_parse_machine() {
        let input = "Button A: X+1, Y+2\nButton B: X+3, Y+4\nPrize: X=5, Y=6\n";
        assert_eq!(
            Machine::parse(input),
            Ok((
                "",
                Machine {
                    button_a: Location::new(1, 2),
                    button_b: Location::new(3, 4),
                    prize: Location::new(5, 6)
                }
            ))
        );
    }

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result.len(), 4);

        assert_eq!(
            result[2],
            Machine {
                button_a: Location::new(17, 86),
                button_b: Location::new(84, 37),
                prize: Location::new(7870, 6450)
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
