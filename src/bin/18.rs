use advent_of_code::utils::location::Location;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_inner(input, Location::new(70, 70), 1024)
}

fn part_one_inner(input: &str, size: Location<i32>, simulate: usize) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(7, 7), 12);
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
