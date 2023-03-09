use std::str::FromStr;

enum Action {
    Rock = 1,
    Paper,
    Scissor,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Action, Self::Err> {
        match s {
            "A" => Ok(Action::Rock),
            "B" => Ok(Action::Paper),
            "C" => Ok(Action::Scissor),
            _ => Err("Unexpected action".to_string()),
        }
    }
}

impl Action {
    fn to_losing_move(&self) -> Action {
        match self {
            Action::Rock => Action::Scissor,
            Action::Paper => Action::Rock,
            Action::Scissor => Action::Paper,
        }
    }

    fn to_winning_move(&self) -> Action {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissor,
            Action::Scissor => Action::Rock,
        }
    }
}

fn solution(input: &str) -> u32 {
    let total = input
        .lines()
        .map(|round| {
            let turn = round.split_whitespace().collect::<Vec<_>>();
            let theirs = turn[0].parse::<Action>().unwrap();

            return match turn[1] {
                "X" => 0 + theirs.to_losing_move() as u32,
                "Y" => 3 + theirs as u32,
                "Z" => 6 + theirs.to_winning_move() as u32,
                _ => panic!("Unexpected desired outcome {}", turn[1]),
            };
        })
        .sum::<u32>();

    return total;
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// github solution
//
// basically exactly the same, it just does the pattern matching
// inline (nested in match turn[1]) instead of adding methods to Action

#[cfg(test)]
mod tests {
    use super::solution;

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, 12)
    }
}
