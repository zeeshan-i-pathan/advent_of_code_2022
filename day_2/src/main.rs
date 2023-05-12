use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Not a valid outcome".to_string()),
        }
    }
}

impl Outcome {
    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
            Outcome::Win => theirs.winning_move(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Not a valid move".to_string()),
        }
    }
}

impl Move {
    // const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    // Putting the Beating combination in one place as
    const MOVE_MATCH: [(Move, Move); 3] = [
        (Move::Rock, Move::Scissors),
        (Move::Paper, Move::Rock),
        (Move::Scissors, Move::Paper),
    ];

    fn beats(self, other: Move) -> bool {
        // // Hardcoded Match other functions need to depend on this function and the ALL_MOVES array
        // matches!(
        //     (self, other),
        //     (Self::Rock, Self::Scissors)
        //         | (Self::Paper, Self::Rock)
        //         | (Self::Scissors, Self::Paper)
        // )

        // // Another way to do beats. but too many get().unwraps
        // if (self, other) == *Self::MOVE_MATCH.get(0).unwrap()
        //     || (self, other) == *Self::MOVE_MATCH.get(1).unwrap()
        //     || (self, other) == *Self::MOVE_MATCH.get(2).unwrap()
        // {
        //     true
        // } else {
        //     false
        // }

        // // I like this way of computing beats
        match Self::MOVE_MATCH
            .iter()
            .find(|tuple| (self, other) == **tuple)
        {
            Some(_) => true,
            None => false,
        }

        // Imperative way also works
        // for beating_moves in Self::MOVE_MATCH {
        //     if (self, other) == beating_moves {
        //         return true;
        //     }
        // }
        // return false;
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn winning_move(self) -> Self {
        // // Looping ALL_MOVES and then calling beats to find beating move
        // *Self::ALL_MOVES
        //     .iter()
        //     // .copied()
        //     .find(|m| m.beats(self))
        //     .unwrap()

        // Doing the same but with a MOVE_MATCH constant
        Self::MOVE_MATCH.iter().find(|m| self == m.1).unwrap().0

        // if self == Self::MOVE_MATCH.get(0).unwrap().1 {
        //     Self::MOVE_MATCH.get(0).unwrap().0
        // } else if self == Self::MOVE_MATCH.get(1).unwrap().1 {
        //     Self::MOVE_MATCH.get(1).unwrap().0
        // } else {
        // Self::MOVE_MATCH.get(2).unwrap().0
        // }

        // match self {
        //     Move::Rock => Move::Paper,
        //     Move::Paper => Move::Scissors,
        //     Move::Scissors => Move::Rock,
        // }
    }

    fn losing_move(self) -> Self {
        // // Looping ALL_MOVES and calling beats to find losing move
        // *Self::ALL_MOVES.iter().find(|m| self.beats(**m)).unwrap()

        // Doing the same but with MOVE_MATCH conts & first item in tuple instead of comparing the entire tuple
        // Need to implement PartialEq on tuple for this to work
        Self::MOVE_MATCH.iter().find(|m| self == m.0).unwrap().1

        // if self == Self::MOVE_MATCH.get(0).unwrap().0 {
        //     Self::MOVE_MATCH.get(0).unwrap().1
        // } else if self == Self::MOVE_MATCH.get(1).unwrap().0 {
        //     Self::MOVE_MATCH.get(1).unwrap().1
        // } else {
        //     Self::MOVE_MATCH.get(2).unwrap().1
        // }

        // match self {
        //     Move::Rock => Move::Scissors,
        //     Move::Paper => Move::Rock,
        //     Move::Scissors => Move::Paper,
        // }
    }

    fn drawing_move(self) -> Self {
        // Nothing to do here (Garbage in Garbage out)
        self
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("expected<theirs>SP<ours>EOF, got {s:?}".to_string());
        };
        Ok(Self {
            theirs: theirs.try_into().unwrap(),
            ours: ours.try_into().unwrap(),
            outcome: ours.try_into().unwrap(),
        })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> u16 {
        self.ours as u16 + self.outcome() as u16
    }
}

fn main() {
    // // Benchmark 1: target/debug/day_2
    // // Time (mean ± σ):       2.2 ms ±   0.3 ms    [User: 0.7 ms, System: 0.5 ms]
    // // Range (min … max):     1.8 ms …   4.3 ms    703 runs
    // let total_score: u8 = include_str!("input.txt")
    //     .lines()
    //     .map(|line| line.parse::<Round>().ok())
    //     // Parsing into Round can fail so filtering out cases that do convert successfully
    //     .filter(|predicate| predicate.is_some())
    //     .map(|round| round.unwrap().our_score())
    //     .sum();
    // println!("Total Score is {total_score}");

    // // Benchmark 1: target/debug/day_2
    // // Time (mean ± σ):       2.2 ms ±   0.2 ms    [User: 0.8 ms, System: 0.6 ms]
    // // Range (min … max):     1.9 ms …   3.3 ms    923 runs
    // let total_score: u8 = include_str!("input.txt")
    //     .lines()
    //     .map(|line| line.parse::<Round>().ok())
    //     // Another way is to map / map and then flatten
    //     .map(|round| round.map(|f| f.our_score()))
    //     .flatten()
    //     .sum();
    // println!("Total Score is {total_score}");

    // // Another way is to convert it into a Vec
    // // Benchmark 1: target/debug/day_2
    // // Time (mean ± σ):       2.2 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
    // // Range (min … max):     1.8 ms …   3.3 ms    925 runs
    // let round: Vec<_> = include_str!("input.txt")
    //     .lines()
    //     .map(Round::from_str)
    //     .collect::<Result<_, _>>()
    //     .unwrap();
    // let total_score: u8 = round.iter().map(|r| r.our_score()).sum();
    // println!("Total Score is {total_score}");

    // // Imperative way to solve this problem
    // // Benchmark 1: target/debug/day_2
    // // Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
    // // Range (min … max):     1.8 ms …   3.1 ms    1021 runs
    // let mut tot_score = 0;
    // for round in include_str!("input.txt").lines().map(Round::from_str) {
    //     match round {
    //         Ok(v) => tot_score += v.our_score() as u8,
    //         _ => println!("Not a valid round"),
    //     }
    // }
    // dbg!(tot_score);

    use itertools::Itertools;
    // // Using itertools::process_results
    // // Benchmark 1: target/debug/day_2
    // // Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
    // // Range (min … max):     1.9 ms …   3.4 ms    890 runs
    // let total_score: u8 = itertools::process_results(
    //     include_str!("input.txt")
    //         .lines()
    //         .map(Round::from_str)
    //         .map(|r| r.map(|r| r.our_score())),
    //     |it| it.sum(),
    // )
    // .unwrap();
    // dbg!(total_score);

    // Using combination of itertools::process_results and map_ok
    // Benchmark 1: target/debug/day_2
    // Time (mean ± σ):       2.1 ms ±   0.2 ms    [User: 0.7 ms, System: 0.5 ms]
    // Range (min … max):     1.8 ms …   3.4 ms    1606 runs
    let part_1_score: u16 = itertools::process_results(
        include_str!("input.txt")
            .lines()
            .map(Round::from_str)
            .map_ok(|r| r.our_score()),
        |it| it.sum(),
    )
    .unwrap();
    dbg!(part_1_score);

    let part_2_score: u16 = itertools::process_results(
        include_str!("input.txt")
            .lines()
            .map(Round::from_str)
            .map_ok(|i| {
                let mut round = i;
                round.ours = round.outcome.matching_move(round.theirs);
                return round.our_score();
            }),
        |it| it.sum(),
    )
    .unwrap();
    dbg!(part_2_score);
}
