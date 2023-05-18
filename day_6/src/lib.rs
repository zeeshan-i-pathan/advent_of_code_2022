trait LowercaseLetter {
    fn to_u32_for_bitset(&self) -> u32;
}

impl LowercaseLetter for u8 {
    fn to_u32_for_bitset(&self) -> u32 {
        1 << (*self as u32 - 'a' as u32)
    }
}

pub fn process(input: &str, dist_char_len: usize) -> usize {
    input
        .as_bytes()
        .windows(dist_char_len)
        .position(|window| {
            window
                .iter()
                .map(|c| c.to_u32_for_bitset())
                .fold(0, |acc, x| acc | x)
                .count_ones()
                == dist_char_len as u32
        })
        .map(|m| m + dist_char_len)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUTS: [&str; 4] = [
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    const ANSWERS: [usize; 4] = [5, 6, 10, 11];

    #[test]
    fn part_1_test() {
        for (idx, input) in INPUTS.iter().enumerate() {
            let result = process(input, 4);
            assert_eq!(result, ANSWERS[idx]);
        }
    }
}
