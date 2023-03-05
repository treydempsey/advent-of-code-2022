use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day01/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> String {
    let max = input
        .split("\n\n")
        .map(|food| {
            food
                .split('\n')
                .map(|c| c.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .max().unwrap();

    max.to_string()
}

fn part2(input: &str) -> String {
    let mut calories = input
        .split("\n\n")
        .map(|food| {
            food
                .split("\n")
                .map(|c| c.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    calories.sort_unstable_by(|a, b| b.cmp(a));
    let max = calories.iter()
        .take(3)
        .sum::<u32>();

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!("24000", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("45000", part2(INPUT));
    }
}
