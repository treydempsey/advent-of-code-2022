use std::collections::HashSet;
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day03/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}


fn part1(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            let a = line[0..(line.len()/2)].chars().collect::<HashSet<_>>();
            let b = line[(line.len()/2)..].chars().collect::<HashSet<_>>();
            (a, b)
        })
        .map(|(a, b)| {
            a.intersection(&b).collect::<String>()
        })
        .map(|s| {
            let c = s.as_bytes()[0] as u32;
            match c {
                65..=90 => c - 38,
                97..=122 => c - 96,
                _ => unimplemented!(),
            }
        })
        .sum::<u32>();

    sum.to_string()
}

fn part2(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| {
            let ab = c[0].intersection(&c[1]).copied().collect();
            c[2].intersection(&ab).collect::<String>()
        })
        .map(|s| {
            let c = s.as_bytes()[0] as u32;
            match c {
                65..=90 => c - 38,
                97..=122 => c - 96,
                _ => unimplemented!(),
            }
        })
        .sum::<u32>();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!("157", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("70", part2(INPUT));
    }
}
