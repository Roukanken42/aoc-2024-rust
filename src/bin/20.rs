use advent_of_code::utils::location::{Access2d, Location};
use advent_of_code::utils::{parse_input_by_lines, Parsable};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;
use num::Zero;
use std::collections::VecDeque;

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Tile::Wall, char('#')),
            value(Tile::Empty, char('.')),
            value(Tile::Start, char('S')),
            value(Tile::End, char('E')),
        ))(input)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    parse_input_by_lines(many1(Tile::parse))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 100)
}

pub fn part_one_inner(input: &str, at_least: u32) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let end = input.iter_2d_keys().find(|&loc| input.get_2d(loc) == Some(&Tile::End))?;

    let mut queue = VecDeque::from(vec![(end, 0)]);
    let mut distances = vec![vec![None; input[0].len()]; input.len()];

    while let Some((loc, dist)) = queue.pop_front() {
        if distances.get_2d(loc) != Some(&None) {
            continue;
        }
        distances.set_2d(loc, Some(dist));

        for neigh in loc.iter_adjacent() {
            if input.get_2d(neigh) != Some(&Tile::Wall) {
                queue.push_back((neigh, dist + 1));
            }
        }
    }

    let cheats = Location::zero()
        .iter_adjacent()
        .into_iter()
        .flat_map(|loc| loc.iter_adjacent().into_iter())
        .filter(|loc| *loc != Location::zero())
        .collect::<Vec<Location<i32>>>();

    Some(
        distances
            .iter_2d_keys()
            .filter(|&loc| if let Some(&Some(_)) = distances.get_2d(loc) { true } else { false })
            .flat_map(|start_loc| cheats.iter().map(move |cheat| (start_loc, *cheat + start_loc)))
            .filter_map(|(start_loc, cheat)| {
                let Some(&Some(loc_dist)) = distances.get_2d(start_loc) else {
                    return None;
                };
                let Some(&Some(cheat_dist)) = distances.get_2d(cheat) else {
                    return None;
                };

                if loc_dist <= cheat_dist + 2 {
                    None
                } else {
                    Some(loc_dist - 2 - cheat_dist)
                }
            })
            .filter(|&dist| dist >= at_least)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_inner(input, 100)
}

pub fn part_two_inner(input: &str, at_least: u32) -> Option<usize> {
    let (_, input) = parse(input).unwrap();

    let end = input.iter_2d_keys().find(|&loc| input.get_2d(loc) == Some(&Tile::End))?;

    let mut queue = VecDeque::from(vec![(end, 0)]);
    let mut distances = vec![vec![None; input[0].len()]; input.len()];

    while let Some((loc, dist)) = queue.pop_front() {
        if distances.get_2d(loc) != Some(&None) {
            continue;
        }
        distances.set_2d(loc, Some(dist));

        for neigh in loc.iter_adjacent() {
            if input.get_2d(neigh) != Some(&Tile::Wall) {
                queue.push_back((neigh, dist + 1));
            }
        }
    }

    let cheats = (-20..=20)
        .flat_map(|x: i32| (-20..=20).flat_map(move |y: i32| if x.abs() + y.abs() <= 20 { Some(Location::new(x, y)) } else { None }))
        .collect::<Vec<Location<i32>>>();

    Some(
        distances
            .iter_2d_keys()
            .filter(|&loc| if let Some(&Some(_)) = distances.get_2d(loc) { true } else { false })
            .flat_map(|start_loc| cheats.iter().map(move |cheat| (start_loc, *cheat + start_loc)))
            .filter_map(|(start_loc, cheat)| {
                let Some(&Some(loc_dist)) = distances.get_2d(start_loc) else {
                    return None;
                };
                let Some(&Some(cheat_dist)) = distances.get_2d(cheat) else {
                    return None;
                };

                let manhattan = start_loc.manhattan_distance(cheat) as u32;

                if loc_dist <= cheat_dist + manhattan {
                    None
                } else {
                    Some(loc_dist - manhattan - cheat_dist)
                }
            })
            .filter(|&dist| dist >= at_least)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "S.#\n\
                     #.E";
        let result = parse(input);

        assert_eq!(
            result,
            Ok((
                "",
                vec![vec![Tile::Start, Tile::Empty, Tile::Wall], vec![Tile::Wall, Tile::Empty, Tile::End],]
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(2 + 3 + 5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), 70);
        assert_eq!(result, Some(12 + 22 + 4 + 3));
    }
}
