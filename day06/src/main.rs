use std::collections::HashSet;
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day06/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = (usize, &str)> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .nth(win_size - 1)
            .map(|(to, c)| (from, &src[from..from + to + c.len_utf8()]))
    })
}

fn part1(input: &str) -> String {
    let mut windows = char_windows(input, 4);

    let v = &windows.find(|(_, win)| {
        let c = win.chars().collect::<HashSet<_>>();
        c.len() == 4
    });
    (v.unwrap_or((0, "")).0 + 4).to_string()
}

fn part2(input: &str) -> String {
    let mut windows = char_windows(input, 14);

    let v = &windows.find(|(_, win)| {
        let c = win.chars().collect::<HashSet<_>>();
        c.len() == 14
    });
    (v.unwrap_or((0, "")).0 + 14).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        let answers = vec![7, 5, 6, 10, 11];
        for (i, line) in INPUT.lines().enumerate() {
            assert_eq!(answers[i].to_string(), part1(line));
        }
    }

    #[test]
    fn test_part2() {
        let answers = vec![19, 23, 23, 29, 26];
        for (i, line) in INPUT.lines().enumerate() {
            assert_eq!(answers[i].to_string(), part2(line));
        }
    }
}
