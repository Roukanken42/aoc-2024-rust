use advent_of_code::utils::parse_input_by_lines;
use nom::bytes::complete::take_while;
use nom::combinator::map;
use nom::IResult;

advent_of_code::solution!(12);

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let line = map(take_while(|c| ('A'..='Z').contains(&c)), |line: &str| line.chars().collect());
    parse_input_by_lines(line)(input)
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
        let (rest, result) = parse(&input).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result[0], vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F']);
        assert_eq!(result[2], vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F']);
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
