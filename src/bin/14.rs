use advent_of_code::utils::location::Location;
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use image::ImageBuffer;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use num::traits::Euclid;
use std::cmp::Ordering::Equal;

advent_of_code::solution!(14);

fn parse_location(input: &str) -> IResult<&str, Location<i32>> {
    map(separated_pair(i32::parse, tag(","), i32::parse), |(x, y)| Location::new(x, y))(input)
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Robot {
    position: Location<i32>,
    velocity: Location<i32>,
}

impl Parsable<'_> for Robot {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, position) = preceded(tag("p="), parse_location)(input)?;
        let (input, _) = space1(input)?;
        let (input, velocity) = preceded(tag("v="), parse_location)(input)?;

        Ok((input, Robot { position, velocity }))
    }
}

impl Robot {
    fn simulate_pos(&self, time: i32, area_size: &Location<i32>) -> Location<i32> {
        (self.position + self.velocity * time).rem_euclid(area_size)
    }

    fn step(&mut self, area_size: &Location<i32>) {
        self.position = self.simulate_pos(1, area_size);
    }

    fn step_by(&mut self, by: i32, area_size: &Location<i32>) {
        self.position = self.simulate_pos(by, area_size);
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    parse_input_by_lines(Robot::parse)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, Location::new(101, 103))
}

pub fn part_one_inner(input: &str, size: Location<i32>) -> Option<usize> {
    let (_, robots) = parse(input).unwrap();

    let res = robots
        .into_iter()
        .map(|robot| robot.simulate_pos(100, &size))
        .map(|pos| (pos.x.cmp(&(size.x / 2)), pos.y.cmp(&(size.y / 2))))
        .filter(|(x, y)| x != &Equal && y != &Equal)
        .counts()
        .iter()
        .fold(1, |acc, (_, &count)| acc * count);

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_inner(input, Location::new(101, 103))
}

#[allow(dead_code)]
pub fn part_two_generate_images(input: &str, size: Location<i32>, max: i32) -> Option<u32> {
    let (_, robots) = parse(input).unwrap();

    let mut robots = robots;

    for iter in 1..max {
        robots.iter_mut().for_each(|robot| robot.step(&size));

        let mut image = ImageBuffer::new(size.x as u32, size.y as u32);
        for robot in robots.iter() {
            image.put_pixel(robot.position.x as u32, robot.position.y as u32, image::Rgb([255u8, 255u8, 255u8]));
        }
        image.save(format!("data/outputs/day14/{:04}.png", iter)).unwrap();
    }

    None
}

pub fn part_two_inner(input: &str, size: Location<i32>) -> Option<usize> {
    let (_, robots) = parse(input).unwrap();

    let find_min = |vec: &Vec<i32>| {
        vec.iter()
            .enumerate()
            .map(|(i, &variance)| (variance, i))
            .min()
            .map(|(_, i)| i)
            .unwrap_or(0)
    };

    let len = robots.len() as i32;

    let mut variances_x = vec![];
    let mut robots_x = robots.iter().copied().collect::<Vec<_>>();
    for _ in 0..size.x {
        let sum = robots_x.iter().map(|robot| robot.position.x).sum::<i32>();
        let mean = sum / len;

        let variance = robots_x.iter().map(|robot| (robot.position.x - mean).pow(2)).sum::<i32>() / len;
        variances_x.push(variance);

        robots_x.iter_mut().for_each(|robot| robot.step(&size));
    }

    let min_x = find_min(&variances_x);

    let mut variances_y = vec![];
    let mut robots_y = robots.iter().copied().collect::<Vec<_>>();
    robots_y.iter_mut().for_each(|robot| robot.step_by(min_x as i32, &size));

    for _ in 0..size.y {
        let sum = robots_y.iter().map(|robot| robot.position.y).sum::<i32>();
        let mean = sum / len;

        let variance = robots_y.iter().map(|robot| (robot.position.y - mean).pow(2)).sum::<i32>() / len;
        variances_y.push(variance);

        robots_y.iter_mut().for_each(|robot| robot.step_by(size.x, &size));
    }

    let min_y = find_min(&variances_y);

    Some(min_x + min_y * size.x as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (rest, robots) = parse(&input).unwrap();

        assert_eq!(rest, "");
        assert_eq!(
            robots[0],
            Robot {
                position: Location::new(0, 4),
                velocity: Location::new(3, -3)
            }
        );
        assert_eq!(
            robots[2],
            Robot {
                position: Location::new(10, 3),
                velocity: Location::new(-1, 2)
            }
        );
    }

    #[test]
    fn test_simulate_pos() {
        let robot = Robot {
            position: Location::new(2, 4),
            velocity: Location::new(2, -3),
        };

        let area_size = Location::new(11, 7);

        assert_eq!(robot.simulate_pos(1, &area_size), Location::new(4, 1));
        assert_eq!(robot.simulate_pos(2, &area_size), Location::new(6, 5));
        assert_eq!(robot.simulate_pos(3, &area_size), Location::new(8, 2));
        assert_eq!(robot.simulate_pos(4, &area_size), Location::new(10, 6));
        assert_eq!(robot.simulate_pos(5, &area_size), Location::new(1, 3));
    }

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), Location::new(11, 7));
        assert_eq!(result, Some(12));
    }
}
