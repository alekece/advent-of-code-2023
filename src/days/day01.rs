const RADIX: u32 = 10;
const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_input() -> &'static str {
    include_str!("../../data/day01.txt")
}

pub fn solve_part1() {
    let solution = get_input()
        .lines()
        .map(|s| {
            let mut digits = s.chars().filter_map(|c| c.to_digit(RADIX));

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);

            (first * RADIX + last) as usize
        })
        .sum::<usize>();

    println!("{solution}");
}

pub fn solve_part2() {
    let solution = get_input()
        .lines()
        .map(|s| {
            let mut digits = s.chars().enumerate().filter_map(|(i, c)| {
                c.to_digit(RADIX).or_else(|| {
                    NUMBERS
                        .iter()
                        .enumerate()
                        .find_map(|(j, number)| s[i..].starts_with(number).then_some((j + 1) as u32))
                })
            });

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);

            (first * RADIX + last) as usize
        })
        .sum::<usize>();

    println!("{solution}");
}
