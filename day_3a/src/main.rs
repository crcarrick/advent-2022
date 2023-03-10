fn solution(input: &str) -> usize {
    return input
        .lines()
        .map(|l| {
            let mut chars = ('a'..='z').chain('A'..='Z').into_iter();
            let (a, b) = l.split_at(l.len() / 2);
            let common = a.chars().filter(|c| b.contains(*c)).next().unwrap();

            return chars.position(|c| c == common).unwrap() + 1;
        })
        .sum();
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// i could have done some byte math (b'b' - b'a' gives you it's position)
// instead of creating the "abcdef...ABCDEF..." lookup string

#[cfg(test)]
mod tests {
    use super::solution;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, 157)
    }
}
