struct Card {
    winning_numbers: Vec<usize>,
    scratched_numbers: Vec<usize>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let parse_integer = |v: &str| v.parse::<usize>().unwrap();

        let (winning_numbers, scratched_numbers) = value
            .split_once('|')
            .map(|(winning_numbers, scratched_numbers)| {
                (
                    winning_numbers.split_whitespace().map(parse_integer).collect(),
                    scratched_numbers.split_whitespace().map(parse_integer).collect(),
                )
            })
            .unwrap();

        Self {
            winning_numbers,
            scratched_numbers,
        }
    }
}

impl Card {
    fn score(&self) -> usize {
        self.scratched_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .fold(0, |acc, _| match acc {
                0 => 1,
                _ => acc * 2,
            })
    }

    fn matching_numbers(&self) -> usize {
        self.scratched_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

fn get_input() -> &'static str {
    include_str!("../../data/day04.txt")
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|s| s.split_once(':').map(|(_, card)| card.into()).unwrap())
        .collect()
}

fn get_solution_part1(cards: Vec<Card>) -> usize {
    cards.iter().map(Card::score).sum()
}

fn get_solution_part2(cards: Vec<Card>) -> usize {
    let mut scratchcards = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        match card.matching_numbers() {
            0 => continue,
            n => {
                let copied_scratchcard = scratchcards[i];

                for scratchcard in scratchcards[i + 1..=i + n].iter_mut() {
                    *scratchcard += copied_scratchcard;
                }
            }
        }
    }

    scratchcards.iter().sum()
}

pub fn solve_part1() {
    let input = get_input();
    let cards = parse_input(input);
    let solution = get_solution_part1(cards);

    println!("{solution}");
}

pub fn solve_part2() {
    let input = get_input();
    let cards = parse_input(input);
    let solution = get_solution_part2(cards);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn it_solves_example_part1() {
        let cards = parse_input(INPUT);
        let solution = get_solution_part1(cards);

        assert_eq!(13, solution);
    }

    #[test]
    fn it_solves_example_part2() {
        let cards = parse_input(INPUT);
        let solution = get_solution_part2(cards);

        assert_eq!(30, solution);
    }
}
