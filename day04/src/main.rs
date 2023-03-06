use std::{env, fs, io, path::Path};
use std::cmp;
use std::convert::TryFrom;
use std::num::ParseIntError;

use nom::IResult;
use nom::character::complete;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::separated_pair;

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day04/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Schedule {
    start: u32,
    end: u32,
}

impl Schedule {
    fn contains(&self, other: &Schedule) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Schedule) -> bool {
        (cmp::max(self.start, other.start) as i32) - (cmp::min(self.end, other.end) as i32) <= 0
    }
}

impl TryFrom<(&str, &str)> for Schedule {
    type Error = ParseIntError;

    fn try_from(input: (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Schedule { start: input.0.parse()?, end: input.1.parse()? })
    }
}

fn parse_schedule(input: &str) -> IResult<&str, Schedule> {
    let (input, schedule) = map_res(separated_pair(digit1, complete::char('-'), digit1), Schedule::try_from)(input)?;
    Ok((input, schedule))
}

fn parse_line(input: &str) -> IResult<&str, (Schedule, Schedule)> {
    let (input, schedules) = separated_pair(parse_schedule, complete::char(','), parse_schedule)(input)?;
    Ok((input, schedules))
}

fn part1(input: &str) -> String {
    let count = input.lines()
        .map(|line| {
            let (_, schedules) = parse_line(line).unwrap();
            schedules
        })
        .map(|schedules| {
            if schedules.0.contains(&schedules.1) || schedules.1.contains(&schedules.0) { 1 } else { 0 }
        })
        .sum::<u32>();

    count.to_string()
}

fn part2(input: &str) -> String {
    let count = input.lines()
        .map(|line| {
            let (_, schedules) = parse_line(line).unwrap();
            schedules
        })
        .map(|schedules| {
            if schedules.0.overlaps(&schedules.1) { 1 } else { 0 }
        })
        .sum::<u32>();

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!("2", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("4", part2(INPUT));
    }
}
