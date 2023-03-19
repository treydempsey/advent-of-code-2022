use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let mut dir = env::current_exe()?;
    dir.pop();
    let dir = dir.join(Path::new("../../day07/input.txt"));

    let input = fs::read_to_string(dir)?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
enum Statement<'a> {
    ChDir(&'a str),
    List,
    Directory(&'a str),
    File(usize, &'a str),
}

#[derive(Debug)]
enum Filesystem<'a> {
    Directory(Option<usize>, &'a str),
    File(usize, &'a str),
}

impl<'a> From<Statement<'a>> for Filesystem<'a> {
    fn from(statement: Statement<'a>) -> Self {
        match statement {
            Statement::Directory(n) => Self::Directory(None, n),
            Statement::File(s, n) => Self::File(s, n),
            _ => panic!("from for type of statement not implemented"),
        }
    }
}

fn path(input: &str) -> IResult<&str, &str> {
    take_while(|c| c != ' ')(input)
}

fn parse_chdir(input: &str) -> IResult<&str, Statement> {
    let (input, (_, dir)) = tuple((tag("$ cd "), path))(input)?;
    Ok((input, Statement::ChDir(dir)))
}

fn parse_list(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, Statement::List))
}

fn parse_dir(input: &str) -> IResult<&str, Statement> {
    let (input, (_, dir)) = tuple((tag("dir "), path))(input)?;
    Ok((input, Statement::Directory(dir)))
}

fn parse_file(input: &str) -> IResult<&str, Statement> {
    let (input, (size, _, name)) = tuple((
        map_res(complete::digit1, |s: &str| s.parse::<usize>()),
        tag(" "),
        path,
    ))(input)?;
    Ok((input, Statement::File(size, name)))
}

fn parse_input(input: &str) -> IResult<&str, Statement> {
    alt((parse_chdir, parse_list, parse_dir, parse_file))(input)
}

fn build_filesystem(input: &str) -> HashMap<String, Vec<Filesystem>> {
    let mut filesystem: HashMap<String, Vec<Filesystem>> = HashMap::new();
    let mut cwd: Vec<&str> = Vec::new();
    for line in input.lines() {
        let (_, statement) = parse_input(line).expect("valid statement");
        match statement {
            Statement::ChDir(t) => {
                if t == "/" {
                    cwd.truncate(0);
                    cwd.push("/");
                } else if t == ".." {
                    cwd.pop();
                } else {
                    cwd.push(t);
                }
            }
            Statement::List => {
                let path = path_join_vec(&cwd);
                match filesystem.get_mut(&path) {
                    Some(_) => panic!("Directory before list"),
                    None => {
                        filesystem.insert(path, vec![Filesystem::Directory(None, ".")]);
                    }
                }
            }
            Statement::Directory(..) | Statement::File(..) => {
                let entry = statement.into();
                let path = path_join_vec(&cwd);
                match filesystem.get_mut(&path) {
                    Some(entries) => entries.push(entry),
                    None => panic!("Directory entry before ."),
                }
            }
        }
    }

    filesystem
}

fn calculate_size(filesystem: &mut HashMap<String, Vec<Filesystem>>) {
    let mut visited: HashSet<String> = HashSet::new();
    let mut dir_stack: Vec<String> = Vec::new();
    dir_stack.push("/".into());
    while !dir_stack.is_empty() {
        let cwd = dir_stack.pop();
        if let Some(cwd) = cwd {
            visited.insert(cwd.clone());
            if let Some(entries) = filesystem.get(&cwd) {
                let children = entries
                    .into_iter()
                    .flat_map(|e| match e {
                        Filesystem::Directory(_, dir) if dir != &"." => {
                            Some(path_join_vec(&vec![&cwd, dir]))
                        }
                        _ => None,
                    })
                    .filter(|d| visited.get(d).is_none())
                    .collect::<Vec<String>>();

                if children.len() > 0 {
                    dir_stack.push(cwd);
                    dir_stack.extend(children);
                } else {
                    let size: usize = entries
                        .into_iter()
                        .flat_map(|e| match e {
                            Filesystem::File(s, _) => Some(s),
                            Filesystem::Directory(Some(s), dir) if dir != &"." => Some(s),
                            _ => None,
                        })
                        .sum();

                    if let Some(entries) = filesystem.get_mut(&cwd) {
                        entries.into_iter().for_each(|e| match e {
                            Filesystem::Directory(s, name) if name == &"." => *s = Some(size),
                            _ => (),
                        });
                    }

                    let (b, d) = path_split(&cwd);
                    if let Some(entries) = filesystem.get_mut(b) {
                        entries.into_iter().for_each(|e| match e {
                            Filesystem::Directory(s, name) if name == &d => *s = Some(size),
                            _ => (),
                        });
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> String {
    let mut filesystem = build_filesystem(input);
    calculate_size(&mut filesystem);

    let r: usize = filesystem
        .into_iter()
        .flat_map(|(_k, v)| {
            v.into_iter().flat_map(|e| match e {
                Filesystem::Directory(Some(sz), d) if d != "." && sz < 100000 => Some(sz),
                _ => None,
            })
        })
        .sum();

    r.to_string()
}

fn part2(input: &str) -> String {
    let mut filesystem = build_filesystem(input);
    calculate_size(&mut filesystem);

    let total_size = if let Some(entries) = filesystem.get("/") {
        entries
            .into_iter()
            .find(|e| match e {
                Filesystem::Directory(Some(_), dir) if dir == &"." => true,
                _ => false,
            })
            .map(|f| match f {
                Filesystem::Directory(Some(sz), _) => *sz,
                _ => 0,
            })
    } else {
        Some(0)
    }
    .unwrap_or(0);
    let needle = 30000000 - (70000000 - total_size);

    let mut target = None;
    let mut target_size = 70000000;

    let mut dir_stack: Vec<String> = Vec::new();
    dir_stack.push("/".into());
    while dir_stack.len() > 0 {
        let cwd = dir_stack.pop();
        if let Some(cwd) = cwd {
            // Check the current directory first
            let (b, d) = path_split(&cwd);
            if let Some(entries) = filesystem.get_mut(b) {
                entries.into_iter().for_each(|e| match e {
                    Filesystem::Directory(Some(s), name) if name == &d => {
                        if *s >= needle && *s < target_size {
                            target = Some(cwd.clone());
                            target_size = *s;
                        }
                    }
                    _ => (),
                });
            }

            // Handle child directories
            if let Some(entries) = filesystem.get(&cwd) {
                let children = entries
                    .into_iter()
                    .flat_map(|e| match e {
                        Filesystem::Directory(_, dir) if dir != &"." => {
                            Some(path_join_vec(vec![cwd, *dir]))
                        }
                        _ => None,
                    })
                    .collect::<Vec<String>>();

                if children.len() > 0 {
                    dir_stack.extend(children);
                }
            }
        }
    }

    target_size.to_string()
}

fn path_split(path: &str) -> (&str, &str) {
    if path == "" || path == "/" {
        ("/", "")
    } else {
        let p = std::path::Path::new(path);
        match p.parent() {
            Some(par) => (
                par.as_os_str().to_str().unwrap_or("/"),
                p.file_name().unwrap().to_str().unwrap_or(""),
            ),
            None => ("/", ""),
        }
    }
}

fn path_join_vec(v: &[&str]) -> String {
    let mut p = std::path::PathBuf::new();
    v.iter().for_each(|c| p.push(c));
    p.as_os_str().to_str().unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!("95437", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("24933642", part2(INPUT));
    }
}
