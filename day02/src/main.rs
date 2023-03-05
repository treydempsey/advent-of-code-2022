use std::str::FromStr;
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day02/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum PlayerMove {
    Elf(Move),
    You(Move),
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for PlayerMove {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Self::Elf(Move::Rock)),
            "B" => Ok(Self::Elf(Move::Paper)),
            "C" => Ok(Self::Elf(Move::Scissors)),
            "X" => Ok(Self::You(Move::Rock)),
            "Y" => Ok(Self::You(Move::Paper)),
            "Z" => Ok(Self::You(Move::Scissors)),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Move {
    fn given(elf_move: &Move, outcome: &Outcome) -> Move {
        match outcome {
            Outcome::Win => match elf_move {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Outcome::Lose => match elf_move {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            Outcome::Draw => match elf_move {
                Self::Rock => Self::Rock,
                Self::Paper => Self::Paper,
                Self::Scissors => Self::Scissors,
            },
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome(&self, other: &Move) -> u32 {
        match self {
            Self::Rock => match other {
                Self::Rock => 3,
                Self::Paper => 0,
                Self::Scissors => 6,
            },
            Self::Paper => match other {
                Self::Rock => 6,
                Self::Paper => 3,
                Self::Scissors => 0,
            },
            Self::Scissors => match other {
                Self::Rock => 0,
                Self::Paper => 6,
                Self::Scissors => 3,
            },
        }
    }
}

impl PlayerMove {
    fn outcome(&self, other: &PlayerMove) -> u32 {
        match (self, other) {
            (Self::Elf(em), Self::You(ym)) => ym.score() + ym.outcome(em),
            (Self::You(ym), Self::Elf(em)) => ym.score() + ym.outcome(em),
            _ => panic!("Can't have a round with an Elf and an Elf or a You and a You"),
        }
    }

    fn given(&self, outcome: &Outcome) -> Self {
        match (self, outcome) {
            (Self::Elf(em), o) => Self::You(Move::given(em, o)),
            _ => panic!("Can't have calculate a game given a You"),
        }
    }
}

fn part1(input: &str) -> String {
    let score = input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|m| m.parse::<PlayerMove>().expect("Error parsing player move"))
                .collect::<Vec<_>>()
        })
        .map(|round| round[0].outcome(&round[1]))
        .sum::<u32>();

    score.to_string()
}

fn part2(input: &str) -> String {
    let score = input
        .lines()
        .map(|l| {
            let instructions = l.split(' ').collect::<Vec<_>>();
            let elf_move = instructions[0]
                .parse::<PlayerMove>()
                .expect("Error parsing elf move");
            let outcome = instructions[1]
                .parse::<Outcome>()
                .expect("Error parsing outcome");
            let your_move = elf_move.given(&outcome);
            vec![elf_move, your_move]
        })
        .map(|round| round[0].outcome(&round[1]))
        .sum::<u32>();

    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!("15", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("12", part2(INPUT));
    }
}
