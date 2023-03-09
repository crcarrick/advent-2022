fn solution(input: &str) -> i32 {
    let elves = input.split("\n\n");

    let totals = elves.map(|elf| {
        return elf
            .split("\n")
            .map(|item| item.parse::<i32>().unwrap())
            .sum();
    });

    let mut most = 0;

    for total in totals {
        if total > most {
            most = total
        }
    }

    return most;
}

fn main() {
    println!("{}", solution(include_str!("../input.txt")))
}

// github solution
//
// let elves = input.split("\n\n");
// let most = elves
//     .map(|elf| {
//         elf.lines()
//             .map(|line| line.parse::<u32>().unwrap())
//             .sum::<u32>()
//     })
//     .max()
//     .unwrap();

// return most;

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

        assert_eq!(result, 24000)
    }
}
