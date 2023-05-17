use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct OverLapPair {
    complete_overlap: usize,
    partial_overlap: usize,
}

impl FromStr for OverLapPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.split(|x| x == ',' || x == '-');
        let (Some(start_1), Some(end_1),Some(start_2), Some(end_2), None) = (chars.next(), chars.next(), chars.next(),chars.next(), chars.next())else {
            println!("Error {s}");
            return Err("Expected number-number,number-number".to_string());
        };
        let start_1: u8 = start_1.parse().unwrap();
        let end_1: u8 = end_1.parse().unwrap();
        let first = start_1..=end_1;
        let start_2: u8 = start_2.parse().unwrap();
        let end_2: u8 = end_2.parse().unwrap();
        let second = start_2..=end_2;
        let mut ret = Self {
            complete_overlap: 0,
            partial_overlap: 0,
        };
        if (first.contains(second.start()) && first.contains(second.end()))
            || (second.contains(first.start()) && second.contains(first.end()))
        {
            ret.complete_overlap = 1;
        }

        if first.contains(second.start())
            || first.contains(second.end())
            || second.contains(first.start())
            || second.contains(first.end())
        {
            ret.partial_overlap = 1;
        }
        Ok(ret)
        // if start_1 <= start_2 {
        //     if end_1 >= end_2 {
        //         return Ok(Self {
        //             complete_overlap: 1,
        //             partial_overlap: 1,
        //         });
        //     } else if end_1 < start_2 {
        //         return Ok(Self {
        //             complete_overlap: 0,
        //             partial_overlap: 0,
        //         });
        //     } else {
        //         return Ok(Self {
        //             complete_overlap: 0,
        //             partial_overlap: 1,
        //         });
        //     }
        // } else if start_2 <= start_1 {
        //     if end_2 >= end_1 {
        //         return Ok(Self {
        //             complete_overlap: 1,
        //             partial_overlap: 1,
        //         });
        //     } else if end_2 < start_1 {
        //         return Ok(Self {
        //             complete_overlap: 0,
        //             partial_overlap: 0,
        //         });
        //     } else {
        //         return Ok(Self {
        //             complete_overlap: 0,
        //             partial_overlap: 1,
        //         });
        //     }
        // } else {
        //     return Ok(Self {
        //         complete_overlap: 0,
        //         partial_overlap: 0,
        //     });
        // }
    }
}

struct Pair {
    first: HashSet<u8>,
    second: HashSet<u8>,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.split(|x| x == ',' || x == '-');
        let (Some(start_1), Some(end_1),Some(start_2), Some(end_2), None) = (chars.next(), chars.next(), chars.next(),chars.next(), chars.next())else {
            println!("Error {s}");
            return Err("Expected number-number,number-number".to_string());
        };
        let start_1: u8 = start_1.parse().unwrap();
        let end_1: u8 = end_1.parse().unwrap();
        let start_2: u8 = start_2.parse().unwrap();
        let end_2: u8 = end_2.parse().unwrap();
        let first: HashSet<u8> = (start_1..=end_1).collect();
        let second: HashSet<u8> = (start_2..=end_2).collect();
        Ok(Self { first, second })
    }
}

pub fn part_1_process(input: &str) -> usize {
    // // Not the most performant way to solve this problem
    // // hyperfine Benchmark 1: target/debug/part_1
    // // Time (mean ± σ):      24.8 ms ±   1.2 ms    [User: 22.7 ms, System: 0.8 ms]
    // // Range (min … max):    21.7 ms …  30.1 ms    120 runs
    // input
    //     .lines()
    //     .map(|line: &str| {
    //         let pair = line.parse::<Pair>().ok().unwrap();
    //         let size = pair.first.intersection(&pair.second).count();
    //         if pair.first.len() == size || pair.second.len() == size {
    //             return 1;
    //         }
    //         0
    //     })
    //     .sum::<usize>()

    // Solves the problem much faster if we don't have to deal with Vec at all
    // hyperfine Benchmark 1: target/debug/part_1
    // Time (mean ± σ):       2.6 ms ±   0.2 ms    [User: 1.2 ms, System: 0.5 ms]
    // Range (min … max):     2.3 ms …   3.7 ms    1193 runs
    input
        .lines()
        .map(|line| {
            let over_lap = line.parse::<OverLapPair>().unwrap_or(OverLapPair {
                complete_overlap: 0,
                partial_overlap: 0,
            });
            over_lap.complete_overlap
        })
        .sum()
}

pub fn part_2_process(input: &str) -> usize {
    // Benchmark 1: target/debug/part_2
    // Time (mean ± σ):       2.6 ms ±   0.2 ms    [User: 1.2 ms, System: 0.5 ms]
    // Range (min … max):     2.3 ms …   3.7 ms    1235 runs
    input
        .lines()
        .map(|line| {
            let over_lap = line.parse::<OverLapPair>().unwrap_or(OverLapPair {
                complete_overlap: 0,
                partial_overlap: 0,
            });
            over_lap.partial_overlap
        })
        .sum()
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
    fn part1_test() {
        let result = part_1_process(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_test() {
        let result: usize = part_2_process(INPUT);
        assert_eq!(result, 4);
    }
}
