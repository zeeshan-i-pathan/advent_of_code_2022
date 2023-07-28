use std::fs::File;
use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir { name: &'a str },
}

fn file(input: &str) -> IResult<&str, Files> {
    dbg!("file", input);
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("abcdefghijklmnopqrstuvwxyz."),
    )(input)?;
    dbg!(input);
    dbg!(size);
    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    dbg!("directory", input);
    let (input, _) = tag("dir ")(input)?;
    dbg!(input);
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir { name }))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    dbg!("ls", input);
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    dbg!(input);
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    dbg!("cd", input);
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        ".." => Operation::Cd(Cd::Up),
        "/" => Operation::Cd(Cd::Root),
        name => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    // let (input, _) = tag("$ ")(input)?;
    // let (input, cmd) = alt((tag("ls"), tag("cd")))(input)?;

    // movind ls / cd to their own parser
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, cmd))
}

pub fn process_part1(input: &str) -> &str {
    let cmds = commands(input).unwrap().1;
    for command in cmds {
        match command {
            Operation::Cd(_) => todo!(),
            Operation::Ls(_) => todo!(),
        }
    }
    ""
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
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "95437");
    }
}
