#![feature(iter_array_chunks)]
#![feature(iter_advance_by)]

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());

    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();

    return (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect();
}

fn solution(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let crates = crates.lines().map(|l| {
        l.chars()
            .enumerate()
            .filter(|&(idx, _)| idx != 0)
            .map(|(_, item)| item.to_string())
            .step_by(4)
            .collect::<Vec<_>>()
    });

    let mut stacks = transpose(crates.rev().collect())
        .into_iter()
        .map(|stack| stack.into_iter().filter(|x| x != " ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", stacks);

    let instructions = instructions
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for instruction in instructions {
        let (num, from, to) = match instruction[..] {
            [a, b, c] => (a, b, c),
            _ => panic!("Shit"),
        };

        (0..num).for_each(|_| {
            let temp = stacks[from - 1].pop().unwrap();

            stacks[to - 1].push(temp);
        })
    }

    return stacks
        .iter()
        .map(|x| x.last().unwrap())
        .fold(String::new(), |a, b| a + b);
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// i feel.. not great about this solution idk

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

        assert_eq!(result, "CMZ")
    }
}
