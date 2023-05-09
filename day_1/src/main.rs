// Just messing around not part of the solution
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

// One possible way to solve the problem
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

// More functional way to solve the ploblem
fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    let groups = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("groups = {groups:?}")
}
