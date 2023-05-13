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
        // .map(|bags| bags.0.chars().find(|item| bags.1.contains(*item)))
        // .map(|mat| Point::try_from(mat.unwrap_or('a') as u8).unwrap().0)
        .sum::<usize>()

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
}
