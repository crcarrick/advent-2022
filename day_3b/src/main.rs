#![feature(iter_array_chunks)]

fn get_alphabet() -> impl Iterator<Item = char> {
    return ('a'..='z').chain('A'..='Z').into_iter();
}

fn solution(input: &str) -> usize {
    return input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_item = a
                .chars()
                .find(|i| b.contains(*i) && c.contains(*i))
                .unwrap_or_else(|| panic!("Could not find common item"));

            get_alphabet()
                .position(|letter| letter == common_item)
                .unwrap()
                + 1
        })
        .sum();
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
//

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

        assert_eq!(result, 70)
    }
}
