use nom::character::complete::{digit1, newline};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use std::collections::HashSet;
use std::env;
use std::io;
use std::fs;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day08/input.txt"));

    let input = fs::read_to_string(dir)?;
    match part1(&input) {
        Ok(r) => println!("{}", r),
        Err(e) => println!("Error: {}", e),
    }
    //println!("part2: {}", part2(&input));
    Ok(())
}

fn line(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, (row, _)) = tuple((digit1, opt(newline)))(input)?;
    let chars = row.chars().flat_map(|c| c.to_digit(10).and_then(|d| u8::try_from(d).ok())).collect();
    Ok((input, chars))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let (input, lines) = many1(line)(input)?;
    Ok((input, lines))
}

fn part1(input: &str) -> Result<String, nom::Err<nom::error::Error<&str>>> {
    let (_, forest) = parse_input(input)?;
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let rows = forest.len();
    let cols = forest[0].len();

    for r in 1..(rows-1) {
        let mut tallest = forest[r][0];
        for c in 1..(cols-1) {
            if forest[r][c] > tallest {
                tallest = forest[r][c];
                visible.insert((c, r));
            }
        }

        let mut tallest = forest[r][cols-1];
        for c in (1..(cols-1)).rev() {
            if forest[r][c] > tallest {
                tallest = forest[r][c];
                visible.insert((c, r));
            }
        }
    }

    for c in 1..(cols-1) {
        let mut tallest = forest[0][c];
        for r in 1..(rows-1) {
            if forest[r][c] > tallest {
                tallest = forest[r][c];
                visible.insert((c, r));
            }
        }

        let mut tallest = forest[rows-1][c];
        for r in (1..(rows-1)).rev() {
            if forest[r][c] > tallest {
                tallest = forest[r][c];
                visible.insert((c, r));
            }
        }
    }

    Ok((2 * (rows - 1) + 2 * (cols - 1) + visible.len()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!("21", part1(INPUT).unwrap());
    }
}
