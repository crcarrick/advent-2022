use std::collections::HashSet;

fn solution(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<_>>();

    let mut out = 0;
    let mut i = 0;

    while i < chars.len() as u32 - 1 {
        let mut j = 0;
        let mut counts = HashSet::new();

        counts.insert(chars[i as usize]);

        while j < 4 {
            counts.insert(chars[(i + j) as usize]);
            j += 1
        }

        if counts.len() == 4 {
            out = i + 4;
            break;
        }

        i += 1
    }

    return out;
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// see notes on 6b

#[cfg(test)]
mod tests {
    use super::solution;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, 7)
    }
}
