fn get_input() -> &'static str {
    return include_str!("../input.txt");
}

fn main() {
    let input = get_input();
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    elves.sort();
    elves.reverse();

    println!("{}", elves.iter().take(3).sum::<u32>());
}

// this is basically what i had
fn _solution_from_github() {
    let input = get_input();
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    elves.sort();

    println!("{}", elves.iter().rev().take(3).sum::<u32>());
}
