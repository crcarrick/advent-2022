fn get_input() -> &'static str {
    return include_str!("../input.txt");
}

#[derive(Copy, Clone)]
enum Action {
    Rock = 1,
    Paper,
    Scissor,
}

impl Action {
    fn from_turn(turn: &str) -> Action {
        match turn {
            "A" | "X" => Action::Rock,
            "B" | "Y" => Action::Paper,
            "C" | "Z" => Action::Scissor,
            _ => panic!("Unknown turn: {}", turn),
        }
    }
}

fn main() {
    let input = get_input();
    let score = input
        .lines()
        .map(|r| {
            let round = r
                .split_whitespace()
                .map(|turn| Action::from_turn(turn))
                .collect::<Vec<_>>();

            let outcome = match &round[..] {
                [Action::Rock, Action::Scissor]
                | [Action::Paper, Action::Rock]
                | [Action::Scissor, Action::Paper] => 0,
                [Action::Rock, Action::Paper]
                | [Action::Paper, Action::Scissor]
                | [Action::Scissor, Action::Rock] => 6,
                [Action::Rock, Action::Rock]
                | [Action::Paper, Action::Paper]
                | [Action::Scissor, Action::Scissor] => 3,
                _ => panic!("Expected 2 actions"),
            };

            return outcome + round[1] as u32;
        })
        .sum::<u32>();

    println!("{}", score)
}

fn _solution_from_github() {
    // overall a similar solution but they used some rust features i wasn't aware of
    //
    // impl FromStr on Action {
    //     type Err = String
    //
    //     from_str(s: &str) -> Result<Self, Self::Err> {
    //         match s {
    //             "A" | "X" => Ok(Action::Rock),
    //             "B" | "Y" => Ok(Action::Paper),
    //             "C" | "Z" => Ok(Action::Scissor),
    //             _ => Err("Not a known action".to_string()),
    //         }
    //     }
    // }
    //
    // "A"::parse<Action>().unwrap()
    //
    // impl PartialOrd on Action {
    //     fn partial_cmp(&self, other: &Self) -> Option<std::comp::Ordering> {
    //         if self == &Action::Scissor && other == &Action::Rock {
    //             return Some(Ordering::Less)
    //         } else if self == &Action::Rock && other == &Action::Scissor {
    //             return Some(Ordering::Greater)
    //         } else {
    //             return Some((*self as u8).cmp(&(*other as u8)))
    //         }
    //     }
    // }
    //
    // match round[0].partial_cmp(round[1]) {
    //     Some(Ordering::Equal) => 3 + round[1] as u32,
    //     Some(Ordering::Less) => 6 + round[1] as u32,
    //     Some(Ordering::Greater) => 0 + round[1] as u32,
    //     None => panic!("Actions should be comparable")
    // }
    //
}
