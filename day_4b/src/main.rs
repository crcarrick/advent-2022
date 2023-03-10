fn solution(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let (l, r) = l.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            let ((a, b), (c, d)) = (
                (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()),
                (c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap()),
            );

            return a <= d && c <= b;
        })
        .count()
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// github solution
//
// idk i couldn't find one but i didn't look that hard and i imagine it's close

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
