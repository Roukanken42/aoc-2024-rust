use advent_of_code::utils::{end_of_file, Parsable};
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::iter::successors;

advent_of_code::solution!(17);

#[derive(Debug, PartialEq, Clone, Copy)]
enum ComboOperand {
    Literal(u64),
    RegA,
    RegB,
    RegC,
    Invalid,
}

impl From<u64> for ComboOperand {
    fn from(value: u64) -> Self {
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
    Bxl(u64),
    Bst(ComboOperand),
    Jnz(u64),
    Bxc(),
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<(u64, u64)> for Instruction {
    fn from(value: (u64, u64)) -> Self {
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

#[derive(Debug, PartialEq, Clone)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    memory: Vec<u64>,
    pointer: usize,
}

impl Parsable<'_> for Computer {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, reg_a) = delimited(tag("Register A: "), u64::parse, line_ending)(input)?;
        let (input, reg_b) = delimited(tag("Register B: "), u64::parse, line_ending)(input)?;
        let (input, reg_c) = delimited(tag("Register C: "), u64::parse, line_ending)(input)?;

        let (input, _) = many1(line_ending)(input)?;
        let (input, memory) = preceded(tag("Program: "), separated_list1(tag(","), u64::parse))(input)?;
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

    fn eval_combo_operand(&self, operand: ComboOperand) -> u64 {
        match operand {
            ComboOperand::Literal(value) => value,
            ComboOperand::RegA => self.reg_a,
            ComboOperand::RegB => self.reg_b,
            ComboOperand::RegC => self.reg_c,
            ComboOperand::Invalid => panic!("Invalid operand"),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Option<u64> {
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

    fn iter_execute(&mut self) -> impl Iterator<Item = u64> + '_ {
        successors(Some(None), move |_| self.read().map(|instruction| self.execute_instruction(instruction))).filter_map(|x| x)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, mut computer) = Computer::parse(input).unwrap();

    Some(computer.iter_execute().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

fn find(program: &[u64], state: (u64, usize)) -> Option<u64> {
    let (a, index) = state;
    let b = *program.get(index).unwrap_or(&0);

    if index == program.len() {
        return Some(a);
    }

    for extra_a in 0..8 {
        let new_a = (a << 3) | extra_a;
        let c = new_a >> (extra_a ^ 1);

        if b != (extra_a ^ 5 ^ c) % 8 || new_a == 0 {
            continue;
        }

        if let Some(r) = find(program, (new_a, index + 1)) {
            return Some(r);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, computer) = Computer::parse(input).unwrap();

    let program = computer.memory.iter().copied().rev().collect::<Vec<_>>();
    find(&program, (0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_computer() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
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
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(202322936867370));
    }
}
