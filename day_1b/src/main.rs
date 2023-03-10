fn solution(input: &str) -> u32 {
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    elves.sort();
    elves.reverse();

    return elves.iter().take(3).sum::<u32>();
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// i could have done rev() on the elves.iter() instead of the vec

#[cfg(test)]
mod test {
    use super::solution;

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
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, 45000)
    }
}
