use advent_of_code::utils::location::direction::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::location::Access2d;
use advent_of_code::utils::parse_input;
use itertools::{iproduct, Itertools};
use nom::character::complete::{anychar, line_ending};
use nom::combinator::map;
use nom::multi::{many0, many_till};
use nom::IResult;

advent_of_code::solution!(4);

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    parse_input(many0(map(many_till(anychar, line_ending), |a| a.0)))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let directions = vec![
        UP,
        UP + RIGHT,
        RIGHT,
        RIGHT + DOWN,
        DOWN,
        DOWN + LEFT,
        LEFT,
        LEFT + UP,
    ];
    let search_for = vec!['X', 'M', 'A', 'S'];

    Some(
        iproduct!(input.iter_2d_keys(), directions.iter())
            .filter(|(start, &direction)| {
                for (loc, char) in start.iter_ray(direction).zip(search_for.iter()) {
                    if input.get_2d(loc) != Some(char) {
                        return false;
                    }
                }

                return true;
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, input) = parse(input).unwrap();

    let directions = vec![
        UP + RIGHT,
        RIGHT + DOWN,
        DOWN + LEFT,
        LEFT + UP,
    ];

    let mut result = 0;

    for start in input.iter_2d_keys() {
        if input.get_2d(start) != Some(&'A') { continue; }

        let neighbours = directions.iter().map(|&direction| input.get_2d(start + direction)).collect::<Vec<_>>();
        let counts = neighbours.iter().counts();

        let has_two_m = counts.get(&Some(&'M')).unwrap_or(&0) == &2;
        let has_two_s = counts.get(&Some(&'S')).unwrap_or(&0) == &2;
        let is_not_a_cross = neighbours == vec![Some(&'M'), Some(&'S'), Some(&'M'), Some(&'S')] || neighbours == vec![Some(&'S'), Some(&'M'), Some(&'S'), Some(&'M')];

        if has_two_s && has_two_m && !is_not_a_cross {
            result += 1;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.0.is_empty());
        assert_eq!(
            result.1[0],
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
