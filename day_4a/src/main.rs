fn solution(input: &str) -> usize {
    return input
        .lines()
        .filter_map(|line| {
            let line = line
                .split(",")
                .flat_map(|g| g.split("-"))
                .collect::<Vec<_>>();

            let mut line: [[u8; 2]; 2] = match line[..] {
                [a, b, c, d] => [
                    [a.parse().unwrap(), b.parse().unwrap()],
                    [c.parse().unwrap(), d.parse().unwrap()],
                ],
                _ => panic!("malformed input"),
            };

            line.sort_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));

            let [[a, b], [c, d]] = line;

            if a >= c && b <= d {
                return Some(());
            }

            return None;
        })
        .count();
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// i could have used .split_once() here to save myself some of the loony
// pattern matching i did
//
// input.lines().filter_map(|l| {
//     let (l, r) = l.split_once(",").unwrap();
//     let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
//
//     let (a, b, c, d) = (
//         a.parse::<u8>().unwrap(),
//         b.parse::<u8>().unwrap(),
//         c.parse::<u8>().unwrap(),
//         d.parse::<u8>().unwrap(),
//     );
//
//     if (a >= c && b <= d) || (a <= c && b >= d) {
//         return Some(())
//     }
//     return None
// }).count();
//

#[cfg(test)]
mod tests {
    use super::solution;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_solution() {
        let result = solution(INPUT);

        assert_eq!(result, 4)
    }
}
