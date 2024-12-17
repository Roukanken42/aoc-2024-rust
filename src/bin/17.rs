use advent_of_code::utils::{end_of_file, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::iter::successors;

advent_of_code::solution!(17);

#[derive(Debug, PartialEq)]
enum ComboOperand {
    Literal(u32),
    RegA,
    RegB,
    RegC,
    Invalid,
}

impl From<u32> for ComboOperand {
    fn from(value: u32) -> Self {
        match value {
            x if x < 4 => ComboOperand::Literal(x),
            4 => ComboOperand::RegA,
            5 => ComboOperand::RegB,
            6 => ComboOperand::RegC,
            _ => ComboOperand::Invalid,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(u32),
    Bst(ComboOperand),
    Jnz(u32),
    Bxc(),
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<(u32, u32)> for Instruction {
    fn from(value: (u32, u32)) -> Self {
        let (opcode, operand) = value;

        match opcode {
            0 => Instruction::Adv(operand.into()),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand.into()),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc(),
            5 => Instruction::Out(operand.into()),
            6 => Instruction::Bdv(operand.into()),
            7 => Instruction::Cdv(operand.into()),
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    memory: Vec<u32>,
    pointer: usize,
}

impl Parsable<'_> for Computer {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, reg_a) = delimited(tag("Register A: "), u32::parse, line_ending)(input)?;
        let (input, reg_b) = delimited(tag("Register B: "), u32::parse, line_ending)(input)?;
        let (input, reg_c) = delimited(tag("Register C: "), u32::parse, line_ending)(input)?;

        let (input, _) = many1(line_ending)(input)?;
        let (input, memory) = preceded(tag("Program: "), separated_list1(tag(","), u32::parse))(input)?;
        let (input, _) = end_of_file(input)?;

        Ok((
            input,
            Computer {
                reg_a,
                reg_b,
                reg_c,
                memory,
                pointer: 0,
            },
        ))
    }
}

impl Computer {
    fn read(&mut self) -> Option<Instruction> {
        if self.pointer + 2 <= self.memory.len() {
            let opcode = self.memory[self.pointer];
            let operand = self.memory[self.pointer + 1];
            self.pointer += 2;

            Some((opcode, operand).into())
        } else {
            None
        }
    }

    fn eval_combo_operand(&self, operand: ComboOperand) -> u32 {
        match operand {
            ComboOperand::Literal(value) => value,
            ComboOperand::RegA => self.reg_a,
            ComboOperand::RegB => self.reg_b,
            ComboOperand::RegC => self.reg_c,
            ComboOperand::Invalid => panic!("Invalid operand"),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Option<u32> {
        match instruction {
            Instruction::Adv(operand) => self.reg_a >>= self.eval_combo_operand(operand),
            Instruction::Bxl(value) => self.reg_b ^= value,
            Instruction::Bst(operand) => self.reg_b = self.eval_combo_operand(operand) % 8,
            Instruction::Jnz(value) => self.pointer = if self.reg_a == 0 { self.pointer } else { value as usize },
            Instruction::Bxc() => self.reg_b ^= self.reg_c,
            Instruction::Out(operand) => return Some(self.eval_combo_operand(operand) % 8),
            Instruction::Bdv(operand) => self.reg_b = self.reg_a >> self.eval_combo_operand(operand),
            Instruction::Cdv(operand) => self.reg_c = self.reg_a >> self.eval_combo_operand(operand),
        }

        None
    }

    fn iter_execute(&mut self) -> impl Iterator<Item = u32> + '_ {
        successors(Some(None), move |_| self.read().map(|instruction| self.execute_instruction(instruction))).filter_map(|x| x)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, mut computer) = Computer::parse(input).unwrap();

    Some(computer.iter_execute().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_computer() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = Computer::parse(input);

        assert_eq!(
            result,
            Ok((
                "",
                Computer {
                    reg_a: 729,
                    reg_b: 0,
                    reg_c: 0,
                    memory: vec![0, 1, 5, 4, 3, 0],
                    pointer: 0
                }
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
