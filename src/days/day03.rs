#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Symbol {
    Digit(u32),
    Gear,
    Undefined,
    Dot,
}

struct Schematic {
    raw_repr: Vec<Vec<Symbol>>,
    numbers: Vec<(Coordinate, Coordinate)>,
    symbols: Vec<Coordinate>,
}

fn get_input() -> &'static str {
    include_str!("../../data/day03.txt")
}

fn parse_input(input: &str) -> Schematic {
    let raw_repr = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| {
                    if let Some(digit) = c.to_digit(10) {
                        Symbol::Digit(digit)
                    } else if c == '.' {
                        Symbol::Dot
                    } else if c == '*' {
                        Symbol::Gear
                    } else {
                        Symbol::Undefined
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let numbers = raw_repr
        .iter()
        .enumerate()
        .flat_map(|(y, symbols)| {
            let x_lim = symbols.len();
            let find_digit = |symbol: &Symbol| matches!(symbol, Symbol::Digit(_));
            let find_symbol = |symbol: &Symbol| !matches!(symbol, Symbol::Digit(_));
            let mut numbers = Vec::default();

            let mut x = 0;

            while x < x_lim {
                let first_x = match symbols.iter().skip(x).position(find_digit) {
                    Some(first_x) => first_x + x,
                    None => return numbers,
                };

                let last_x = match symbols.iter().skip(first_x + 1).position(find_symbol) {
                    Some(last_x) => last_x + first_x,
                    None => x_lim - 1,
                };

                numbers.push((
                    Coordinate::new(first_x as i32, y as i32),
                    Coordinate::new(last_x as i32, y as i32),
                ));

                x = last_x + 1;
            }

            numbers
        })
        .collect();

    let symbols = raw_repr
        .iter()
        .enumerate()
        .flat_map(|(y, symbols)| {
            symbols.iter().enumerate().filter_map(move |(x, symbol)| {
                matches!(symbol, Symbol::Undefined | Symbol::Gear).then_some(Coordinate::new(x as i32, y as i32))
            })
        })
        .collect();

    Schematic {
        raw_repr,
        numbers,
        symbols,
    }
}

fn get_adjacent_coordinates(Coordinate { x, y }: Coordinate) -> impl Iterator<Item = Coordinate> {
    [
        Coordinate::new(x - 1, y - 1),
        Coordinate::new(x - 1, y),
        Coordinate::new(x - 1, y + 1),
        Coordinate::new(x, y - 1),
        Coordinate::new(x, y + 1),
        Coordinate::new(x + 1, y - 1),
        Coordinate::new(x + 1, y),
        Coordinate::new(x + 1, y + 1),
    ]
    .into_iter()
    .filter(|coordinate| coordinate.y >= 0 && coordinate.x >= 0)
}

fn get_solution_part1(schematic: Schematic) -> usize {
    schematic
        .numbers
        .iter()
        .filter_map(|number| {
            let y = number.0.y;

            (number.0.x..=number.1.x)
                .any(|x| {
                    get_adjacent_coordinates(Coordinate { x, y }).any(|coordinate| {
                        matches!(
                            schematic
                                .raw_repr
                                .get(coordinate.y as usize)
                                .and_then(|symbols| symbols.get(coordinate.x as usize)),
                            Some(Symbol::Undefined | Symbol::Gear)
                        )
                    })
                })
                .then(|| {
                    (number.0.x..=number.1.x).fold(0usize, |acc, x| match schematic.raw_repr[y as usize][x as usize] {
                        Symbol::Digit(v) => acc * 10 + v as usize,
                        _ => unreachable!(),
                    })
                })
        })
        .sum::<usize>()
}

fn get_solution_part2(schematic: Schematic) -> usize {
    schematic
        .symbols
        .iter()
        .filter(|Coordinate { x, y }| matches!(schematic.raw_repr[*y as usize][*x as usize], Symbol::Gear))
        .filter_map(|coordinate| {

            let adjacent_coordinates = get_adjacent_coordinates(*coordinate).collect::<Vec<_>>();
            let adjacent_numbers = schematic
                .numbers
                .iter()
                .filter(|number| {
                    (number.0.x..=number.1.x)
                        .any(|x| adjacent_coordinates.iter().any(|c| *c == Coordinate::new(x, number.0.y)))
                })
                .collect::<Vec<_>>();

            if adjacent_numbers.len() == 2 {
                let gear_ratio = adjacent_numbers
                    .into_iter()
                    .map(|number| {
                        let y = number.0.y;

                        (number.0.x..=number.1.x).fold(0usize, |acc, x| {
                            match schematic.raw_repr[y as usize][x as usize] {
                                Symbol::Digit(v) => acc * 10 + v as usize,
                                _ => unreachable!(),
                            }
                        })
                    })
                    .product::<usize>();

                Some(gear_ratio)
            } else {
                None
            }
        })
        .sum::<usize>()
}

pub fn solve_part1() {
    let input = get_input();
    let schematic = parse_input(input);
    let solution = get_solution_part1(schematic);

    println!("{solution}");
}

pub fn solve_part2() {
    let input = get_input();
    let schematic = parse_input(input);
    let solution = get_solution_part2(schematic);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_solves_example_part1() {
        let schematic = parse_input(INPUT);
        let solution = get_solution_part1(schematic);

        assert_eq!(4361, solution);
    }

    #[test]
    fn it_solves_example_part2() {
        let schematic = parse_input(INPUT);
        let solution = get_solution_part2(schematic);

        assert_eq!(467835, solution);
    }
}
