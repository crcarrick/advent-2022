fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(l, r)| (l.split_once("-").unwrap(), r.split_once("-").unwrap()))
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// github solution
//
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

        assert_eq!(result, 7)
    }
}
