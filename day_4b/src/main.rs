trait TupleMap<T, U> {
    fn map<F>(&self, f: F) -> (U, U, U, U)
    where
        F: Fn(&T) -> U;
}

impl<T, U> TupleMap<T, U> for (T, T, T, T) {
    fn map<F>(&self, f: F) -> (U, U, U, U)
    where
        F: Fn(&T) -> U,
    {
        let (a, b, c, d) = &self;

        return (f(a), f(b), f(c), f(d));
    }
}

fn solution(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let (l, r) = l.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            let (a, b, c, d) = (a, b, c, d).map(|i| i.parse::<u8>().unwrap());

            return a <= d && c <= b;
        })
        .count()
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// notes
//
// i decided to implement this TupleMap thing for fun
// you wouldn't do that otherwise

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
