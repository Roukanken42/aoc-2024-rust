use advent_of_code::utils::end_of_file;
use nom::bytes::complete::take_while;
use nom::IResult;

advent_of_code::solution!(9);

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = take_while(|c| ('0'..='9').contains(&c))(input)?;
    let (input, _) = end_of_file(input)?;

    let digits = digits.chars().map(|c| c.to_digit(10).unwrap()).collect();
    Ok((input, digits))
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
        assert_eq!(result, vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]);
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
