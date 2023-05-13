trait Score {
    fn score(self) -> usize;
}

impl Score for char {
    fn score(self) -> usize {
        match self {
            ('a'..='z') => self as usize + 1 - b'a' as usize,
            ('A'..='Z') => self as usize + 27 - b'A' as usize,
            _ => 0,
        }
    }
}

pub fn part_1_score(input: &str) -> usize {
    // Hyperfine Benchmark 1: target/debug/part_1
    // Time (mean ± σ):       2.2 ms ±   0.2 ms    [User: 0.9 ms, System: 0.5 ms]
    // Range (min … max):     1.9 ms …   3.2 ms    1369 runs
    input
        .lines()
        .map(|line| {
            let (bag_1, bag_2) = line.split_at(line.len() / 2);
            bag_1
                .chars()
                .find(|item| bag_2.contains(*item))
                .unwrap_or_default()
                .score()
        })
        .sum::<usize>()

    // // Hyperfine Benchmark 1: target/debug/part_1
    // // Time (mean ± σ):       2.2 ms ±   0.2 ms    [User: 0.8 ms, System: 0.5 ms]
    // // Range (min … max):     1.9 ms …   3.2 ms    1336 runs
    // let mut score: usize = 0;
    // for line in input.lines() {
    //     let mid: usize = line.len() / 2;
    //     let (bag_1, bag_2) = (&line[..mid], &line[mid..]);
    //     for char in bag_1.chars() {
    //         if bag_2.contains(char) {
    //             let points = match char {
    //                 ('a'..='z') => char as usize + 1 - 'a' as usize,
    //                 ('A'..='Z') => char as usize + 27 - 'A' as usize,
    //                 _ => 0,
    //             };
    //             score += points;
    //             break;
    //         }
    //     }
    // }
    // score
}

pub fn part_2_score(input: &str) -> usize {
    // let slice = ['l', 'o', 'r', 'e', 'm'];
    // let mut iter = slice.chunks(2);
    // dbg!(iter.next());

    // Hyperfine Benchmark 1: target/debug/part_2
    // Time (mean ± σ):       2.2 ms ±   0.2 ms    [User: 0.8 ms, System: 0.5 ms]
    // Range (min … max):     1.9 ms …   3.3 ms    1229 runs
    let line_vec: Vec<&str> = input.lines().collect();
    let mut rucksacks = line_vec.chunks(3);
    let mut score = 0;
    loop {
        match rucksacks.next() {
            Some(rucksack) => {
                score += rucksack[0]
                    .chars()
                    .find(|item| rucksack[1].contains(*item) && rucksack[2].contains(*item))
                    .unwrap_or_default()
                    .score();
            }
            None => break,
        };
    }
    return score;
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
    fn part1_test() {
        let result = part_1_score(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_test() {
        let result = part_2_score(INPUT);
        assert_eq!(result, 70);
    }
}
