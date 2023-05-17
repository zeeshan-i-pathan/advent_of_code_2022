use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use smallvec::SmallVec;

#[derive(Debug)]
struct Crate(char);

fn parse_crate(i: &str) -> IResult<&str, char> {
    let first_char = |s: &str| s.chars().next().unwrap();
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<char>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<char>>> {
    // let (mut i, c) = parse_crate_or_hole(i)?;
    // let mut v = vec![c];
    // loop {
    //     let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
    //     match maybe_c {
    //         Some(c) => v.push(c),
    //         None => break,
    //     }
    //     i = next_i
    // }

    // Ok((i, v))

    // Simplifying the above complicated logic with this one line
    separated_list1(tag(" "), parse_crate_or_hole)(i)
}

fn rev_transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            // Vec::with_capacity will help us get rid of the reallocations that would have happened while moving crates around when we apply instructions
            let mut v = Vec::with_capacity(256);
            v.extend(iters.iter_mut().rev().filter_map(|n| n.next().unwrap()));
            v
            // .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_num(i: &str) -> IResult<&str, usize> {
    let f = |s: &str| s.parse::<usize>();
    map_res(digit1, f)(i)
}

fn parse_pile_num(i: &str) -> IResult<&str, usize> {
    map(parse_num, |i| i - 1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

trait AppInstruction {
    fn instruct_part_1(&mut self, ins: Instruction);
    fn instruct_part_2(&mut self, ins: Instruction);
}

impl AppInstruction for Vec<Vec<char>> {
    fn instruct_part_1(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self[ins.src].pop().unwrap();
            self[ins.dst].push(el);
        }
    }

    fn instruct_part_2(&mut self, ins: Instruction) {
        // let mut a = (0..ins.quantity)
        for c in (0..ins.quantity)
            .map(|_| self[ins.src].pop().unwrap())
            // .collect::<Vec<char>>();
            // Using SmallVec for Stack Allocation because we are never moving more than 35 items
            .collect::<SmallVec<[char; 64]>>()
            .into_iter()
            .rev()
        {
            // for _ in 0..ins.quantity {
            // let el = a.pop().unwrap();
            self[ins.dst].push(c);
        }
    }
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_num),
            preceded(tag(" from "), parse_pile_num),
            preceded(tag(" to "), parse_pile_num),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut lines = input.lines();
    let crate_lines = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, crate_line)| crate_line)
        })
        .collect();
    dbg!(lines.next().unwrap().is_empty());
    // dbg!(&crate_lines);
    let crate_columns = rev_transpose(crate_lines);
    // dbg!(&crate_columns);
    // for (index, col) in crate_columns.iter().enumerate() {
    //     println!("Pile {index} - {col:?}");
    // }

    let instructions: Vec<Instruction> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    (crate_columns, instructions)
}

pub fn part1_process(input: &str) -> String {
    // let mut crate_lines = vec![];
    // for line in input.lines() {
    //     if let Ok((rest, crate_line)) = all_consuming(parse_crate_line)(line).finish() {
    //         crate_lines.push(crate_line);
    //     }
    // }
    // let mut lines = input.lines();
    // let crate_lines = (&mut lines)
    //     .map_while(|line| {
    //         all_consuming(parse_crate_line)(line)
    //             .finish()
    //             .ok()
    //             .map(|(_, crate_line)| crate_line)
    //     })
    //     .collect();
    // dbg!(lines.next().unwrap().is_empty());
    // // dbg!(&crate_lines);
    // let mut crate_columns = rev_transpose(crate_lines);
    // // dbg!(&crate_columns);
    // // for (index, col) in crate_columns.iter().enumerate() {
    // //     println!("Pile {index} - {col:?}");
    // // }

    // let instructions: Vec<Instruction> = lines
    //     .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
    //     .collect();
    let (mut crate_columns, instructions) = parse_input(input);
    for ins in instructions {
        // println!("{ins:?}");
        crate_columns.instruct_part_1(ins);
    }
    return crate_columns
        .iter()
        .map(|pile| pile.last().unwrap())
        .collect::<String>();
}

pub fn part2_process(input: &str) -> String {
    let (mut crate_columns, instructions) = parse_input(input);
    for ins in instructions {
        crate_columns.instruct_part_2(ins);
    }
    return crate_columns
        .iter()
        .map(|pile| pile.last().unwrap())
        .collect::<String>();
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
    fn part_1_test() {
        let result = part1_process(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_2_test() {
        let result = part2_process(INPUT);
        assert_eq!(result, "MCD");
    }
}

#[test]
fn parse_crate_test() {
    assert_eq!(Ok(("", 'D')), parse_crate("[D]"));
}

#[test]
fn parse_hole_test() {
    assert_eq!(parse_hole("   "), Ok(("", ())));
}

#[test]
fn parse_crate_or_hole_test() {
    assert_eq!(parse_crate_or_hole("   "), Ok(("", None,),));
    assert_eq!(parse_crate_or_hole("[F]"), Ok(("", Some('F'),),));
    assert_eq!(opt(parse_crate_or_hole)(""), Ok(("", None)));
}

#[test]
fn parse_crate_line_test() {
    assert_eq!(
        parse_crate_line("[F]     [G]"),
        Ok(("", vec![Some('F',), None, Some('G',),],),)
    );
}
