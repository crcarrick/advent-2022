fn get_input() -> &'static str {
    return include_str!("../input.txt");
}

fn main() {
    let input = get_input();
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

    println!("{}", most)
}

fn _solution_from_github() {
    let input = get_input();
    let elves = input.split("\n\n");
    let most = elves
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{}", most)
}
