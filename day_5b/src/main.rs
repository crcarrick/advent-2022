use std::collections::VecDeque;

#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<String>>,
}

impl Crates {
    fn from_str(s: &str) -> Crates {
        let stacks = s
            .lines()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|&(i, _)| i != 0)
                    .map(|(_, c)| c.to_string())
                    .step_by(4)
                    .collect()
            })
            .collect();

        return Crates { stacks };
    }

    fn to_output(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|i| i.last().unwrap())
            .fold(String::new(), |a, b| a + b);
    }

    fn rotate(&mut self) -> &mut Self {
        assert!(!self.stacks.is_empty());

        self.stacks.pop();
        self.stacks.reverse();

        // transpose
        self.stacks = self.stacks[0]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                self.stacks
                    .iter()
                    .enumerate()
                    .map(|(j, _)| self.stacks[j][i].clone())
                    .collect()
            })
            .collect();

        return self;
    }

    fn trim(&mut self) -> &mut Self {
        for i in 0..self.stacks.len() {
            loop {
                let item = self.stacks[i].pop().unwrap();

                if item != " " {
                    self.stacks[i].push(item);
                    break;
                }
            }
        }

        return self;
    }

    fn swap(&mut self, instructions: &Instructions) -> &mut Self {
        assert!(!self.stacks.is_empty());

        for i in &instructions.instructions {
            let mut crates = VecDeque::new();

            for _ in 0..i.num {
                let tmp = self.stacks[i.from - 1].pop().unwrap();

                crates.push_front(tmp);
            }

            for c in crates {
                self.stacks[i.to - 1].push(c);
            }
        }

        return self;
    }
}

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_vector(v: Vec<usize>) -> Instruction {
        let (num, from, to) = match v[..] {
            [a, b, c] => (a, b, c),
            _ => panic!("Expected vector with 3 items but got {}", v.len()),
        };

        return Instruction { num, from, to };
    }
}

struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn from_str(s: &str) -> Instructions {
        return Instructions {
            instructions: s
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .filter_map(|i| i.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .map(|l| Instruction::from_vector(l))
                .collect::<Vec<_>>(),
        };
    }
}

fn solution(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    return Crates::from_str(crates)
        .rotate()
        .trim()
        .swap(&Instructions::from_str(instructions))
        .to_output();
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// i added a bunch of data structures here just to learn
// i also goofed around with some method chaining etc
// i also used some imperative code since we've mostly been using
// functional style chains in the rest of the solutions

#[cfg(test)]
mod tests {
    use super::solution;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, "MCD")
    }
}
