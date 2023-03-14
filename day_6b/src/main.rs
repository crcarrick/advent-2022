use std::collections::HashSet;

fn solution(input: &str) -> u32 {
    let chars = input.chars().collect::<Vec<_>>();

    let mut out = 0;
    let mut i = 0;

    while i < chars.len() as u32 - 1 {
        let mut j = 0;
        let mut counts = HashSet::new();

        counts.insert(chars[i as usize]);

        while j < 14 {
            counts.insert(chars[(i + j) as usize]);
            j += 1
        }

        if counts.len() == 14 {
            out = i + 14;
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
// solution found on github is the same but just fancier features
// vec has a `windows` method i didn't know about that returns an iterator
// of moving windows (what i did with the loops).  they used that + a set to
// find a window where all characters were unique

// fn solution(input: &str) -> u32 {
//     let window_size = 14;
//
//     let chars = input.chars().collect::<Vec<_>>();
//     let (start, _) = chars
//         .windows(window_size)
//         .enumerate()
//         .find(|(_, slice)| {
//             let set =
//                 slice.iter().collect::<HashSet<&char>>();
//             slice.len() == set.len()
//         })
//         .unwrap();
//
//     return (start + window_size) as u32;
// }

#[cfg(test)]
mod tests {
    use super::solution;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, 19)
    }
}
