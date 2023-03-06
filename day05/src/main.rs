use std::collections::VecDeque;
use std::convert::TryFrom;
use std::num::ParseIntError;
use std::{env, fs, io, path::Path};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{anychar, digit1, line_ending, not_line_ending, space1};
use nom::combinator::{eof, opt};
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day05/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Instruction {
    num: u32,
    source: usize,
    destination: usize,
}

impl TryFrom<(&str, &str, &str)> for Instruction {
    type Error = ParseIntError;

    fn try_from(input: (&str, &str, &str)) -> Result<Self, Self::Error> {
        Ok(Instruction {
            num: input.0.parse()?,
            source: input.1.parse::<usize>()? - 1,
            destination: input.2.parse::<usize>()? - 1,
        })
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<VecDeque<char>>, Vec<Instruction>)> {
    let (input, rows) = many1(parse_crates)(input)?;
    let (input, _) = terminated(not_line_ending, count(line_ending, 2))(input)?;
    let (input, instructions) = many1(parse_instruction)(input)?;
    let (input, _) = eof(input)?;

    let mut crates: Vec<VecDeque<char>> = Vec::with_capacity(rows[0].len());
    for _ in 0..rows[0].len() {
        crates.push(VecDeque::new());
    }

    for row in rows.into_iter() {
        for (col, c) in row.into_iter().enumerate() {
            if c != ' ' {
                crates[col].push_front(c)
            }
        }
    }

    Ok((input, (crates, instructions)))
}

fn parse_crates(input: &str) -> IResult<&str, Vec<char>> {
    let (input, crates) = separated_list1(complete::char(' '), parse_crate)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, crates))
}

fn parse_crate(input: &str) -> IResult<&str, char> {
    Ok(alt((
        delimited(complete::char('['), anychar, complete::char(']')),
        delimited(
            complete::char(' '),
            complete::char(' '),
            complete::char(' '),
        ),
    ))(input)?)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, num) = preceded(tuple((tag("move"), space1)), digit1)(input)?;
    let (input, source) = preceded(tuple((space1, tag("from"), space1)), digit1)(input)?;
    let (input, destination) = preceded(tuple((space1, tag("to"), space1)), digit1)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((
        input,
        Instruction::try_from((num, source, destination)).unwrap(),
    ))
}

fn part1(input: &str) -> String {
    let (_, (mut crates, instructions)) = parse_input(input).unwrap();

    for instruction in instructions {
        for _ in 0..(instruction.num) {
            let c = crates[instruction.source].pop_back().unwrap();
            crates[instruction.destination].push_back(c);
        }
    }

    crates
        .into_iter()
        .map(|mut c| c.pop_back().unwrap_or(' '))
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let (_, (mut crates, instructions)) = parse_input(input).unwrap();

    for instruction in instructions {
        let split = crates[instruction.source].len() - (instruction.num as usize);
        let mut c = crates[instruction.source].split_off(split);
        crates[instruction.destination].append(&mut c);
    }

    crates
        .into_iter()
        .map(|mut c| c.pop_back().unwrap_or(' '))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!("CMZ", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("MCD", part2(INPUT));
    }
}
