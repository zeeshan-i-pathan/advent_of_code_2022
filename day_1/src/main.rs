// // Just messing around not part of the solution
// use std::fmt;

// struct PathedIOError {
//     path: String,
//     inner: std::io::Error,
// }

// use color_eyre::eyre::Context;

// impl fmt::Debug for PathedIOError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "for file {}: {}", self.path, self.inner)
//     }
// }
// fn main() -> color_eyre::Result<()> {
//     color_eyre::install()?;
//     let input = read_file()?;
//     print!("{input}");
//     Ok(())
// }

// fn read_file() -> Result<String, std::io::Error> {
//     let path = "input.txt";
//     // let input = std::fs::read_to_string(path).unwrap();
//     // fs_err::read_to_string(path)
//     std::fs::read_to_string(path)
// }

// fn read_file() -> color_eyre::Result<String> {
//     let path = "input";
//     let msg = "reading src/input";
//     let input = std::fs::read_to_string(path).wrap_err(msg)?;
//     Ok(input)
// }

// // One possible way to solve the problem
// // Benchmarked with Hyperfine
// // hyperfine ./target/release/day_1 --shell=none --warmup 100
// // Benchmark 1: ./target/release/day_1
// //   Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
// //   Range (min … max):     1.8 ms …   3.1 ms    1386 runs
// fn main() {
//     let mut max = 0;
//     for group in include_str!("input.txt")
//         .replace("\r\n", "\n")
//         .split("\n\n")
//     {
//         let mut sum = 0;
//         for line in group.lines() {
//             sum += line.parse::<i64>().unwrap();
//         }
//         if sum > max {
//             max = sum;
//         }
//     }
//     println!("Max is {max}");
// }

// // More functional way to solve the ploblem
// // Benchmarked with Hyperfine
// // hyperfine ./target/release/day_1 --shell=none --warmup 10
// // Benchmark 1: ./target/release/day_1
// //   Time (mean ± σ):       2.2 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
// //   Range (min … max):     1.8 ms …   3.5 ms    1250 runs
// fn main() {
//     let lines = include_str!("input.txt")
//         .lines()
//         .map(|v| v.parse::<u64>().ok())
//         .collect::<Vec<_>>();
//     // let groups = lines
//     //     .split(|line| line.is_none())
//     //     .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
//     //     .collect::<Vec<_>>();
//     // println!("groups = {groups:?}");
//     let max = lines
//         .split(|line| line.is_none())
//         .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
//         .max()
//         .unwrap_or(0);
//     println!("Max is {max}");
// }

// // Solving it without the need to create a Collection as that is not efficient on a large dataset
// // This solution will work but too convulated
// // Struct on which an Iterator is implemented
// // Benchmarked with Hyperfine
// // hyperfine ./target/release/day_1 --shell=none --warmup 10
// // Benchmark 1: ./target/release/day_1
// // Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
// // Range (min … max):     1.8 ms …   3.3 ms    1498 runs
// struct GroupSumIter<I> {
//     inner: I,
// }

// impl<I> Iterator for GroupSumIter<I>
// where
//     I: Iterator<Item = Option<u64>>,
// {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         // Will reinitialize these variables after every break;
//         let mut sum = 0;
//         let mut last_item = false;
//         loop {
//             match self.inner.next() {
//                 Some(Some(v)) => {
//                     sum += v;
//                     // The next encountered None will be the Last item on the array and should break the loop to return the calculated sum
//                     last_item = true;
//                 }
//                 Some(None) => {
//                     break Some(sum);
//                 }
//                 None => {
//                     if last_item {
//                         break Some(sum);
//                     }
//                     // on the next None Exit the next function
//                     return None;
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     let lines = include_str!("input.txt")
//         .lines()
//         .map(|v| v.parse::<u64>().ok());
//     let elven_lead = GroupSumIter { inner: lines }.max();
//     println!("{elven_lead:?}");
// }

// // Solving it using ItelTools Crate
use itertools::Itertools;
// // Most eligant way to solve this problem :)
// // Benchmarked with Hyperfine
// // hyperfine ./target/release/day_1 --shell=none --warmup 100
// // Benchmark 1: ./target/release/day_1
// //   Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
// //   Range (min … max):     1.9 ms …   3.0 ms    1289 runs

fn main() {
    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok()) // batching method is from the itertools crarte
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|&v| std::cmp::Reverse(v));

    // Solution to Part 1
    let max = answer.clone().max().unwrap_or_default();
    println!("Part 1 Solution {max}");

    // Solution to Part 2
    let answer = answer.take(3).sum::<u64>();
    println!("Part 2 Solution {answer:?}");
}
// // Benchmarked with Hyperfine
// // hyperfine ./target/release/day_1 --shell=none --warmup 100
// // Benchmark 1: ./target/release/day_1
// //   Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
// //   Range (min … max):     1.9 ms …   3.0 ms    1326 runs
// fn main() {
//     let lines = include_str!("input.txt")
//         .lines()
//         .map(|v| v.parse::<u64>().ok())
//         // coalesce method is from the itertools crate
//         .coalesce(|a, b| match (a, b) {
//             (None, None) => Ok(None),
//             (None, Some(b)) => Ok(Some(b)),
//             (Some(a), Some(b)) => Ok(Some(a + b)),
//             (Some(a), None) => Err((Some(a), None)),
//         })
//         .max()
//         .flatten()
//         .unwrap_or(0);
//     println!("{lines:?}");
// }
