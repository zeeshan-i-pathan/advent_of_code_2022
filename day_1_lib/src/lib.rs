pub fn process_part1(input: &str) -> u64 {
    let result: u64 = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap();
    result
}

pub fn process_part2(input: &str) -> u64 {
    let mut result = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    let sum = result.iter().take(3).sum::<u64>();
    // dbg!(result);
    sum
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
    fn part1_test() {
        let result = process_part1(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_test() {
        let result = process_part2(INPUT);
        assert_eq!(result, 45000);
    }
}
