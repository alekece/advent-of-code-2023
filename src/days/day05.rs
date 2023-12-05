use std::ops::Range;

#[derive(Debug)]
struct Seeds(Vec<usize>);

impl From<&str> for Seeds {
    fn from(s: &str) -> Self {
        let (_, s) = s.split_once(':').unwrap();

        let seeds = s
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Self(seeds)
    }
}

#[derive(Debug)]
struct Map(Vec<MapEntry>);

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let entries = s.trim().split('\n').skip(1).map(MapEntry::from).collect();

        Map(entries)
    }
}

impl Map {
    fn transform(&self, source: usize) -> usize {
        self.0
            .iter()
            .flat_map(|entry| entry.transform(source))
            .next()
            .unwrap_or(source)
    }
}

#[derive(Debug)]
struct MapEntry {
    destination: Range<usize>,
    source: Range<usize>,
}

impl From<&str> for MapEntry {
    fn from(s: &str) -> Self {
        let values = s
            .split_whitespace()
            .take(3)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        MapEntry {
            destination: values[0]..(values[0] + values[2]),
            source: values[1]..(values[1] + values[2]),
        }
    }
}

impl MapEntry {
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn transform(&self, source: usize) -> Option<usize> {
        self.source
            .contains(&source)
            // must be kept to avoid subtrat with overflow due to eager initialization with `then_some`
            .then(|| self.destination.start + (source - self.source.start))
    }
}

#[derive(Debug)]
struct Almanax {
    seeds: Seeds,
    maps: Vec<Map>,
}

fn get_input() -> &'static str {
    include_str!("../../data/day05.txt")
}

fn parse_input(input: &str) -> Almanax {
    let input = input.split("\n\n").collect::<Vec<_>>();

    let seeds = input[0].into();
    let maps = input[1..].iter().map(|s| Map::from(*s)).collect();

    Almanax { seeds, maps }
}

fn get_solution_part1(almanax: Almanax) -> usize {
    almanax
        .seeds
        .0
        .iter()
        .map(|seed| almanax.maps.iter().fold(*seed, |source, map| map.transform(source)))
        .min()
        .unwrap()
}

fn get_solution_part2(almanax: Almanax) -> usize {
    almanax
        .seeds
        .0
        .chunks(2)
        // definitely not the quickest solution but who cares ?
        .flat_map(|v| v[0]..(v[0] + v[1]))
        .map(|seed| almanax.maps.iter().fold(seed, |source, map| map.transform(source)))
        .min()
        .unwrap()
}

pub fn solve_part1() {
    let input = get_input();
    let almanax = parse_input(input);
    let solution = get_solution_part1(almanax);

    println!("{solution}");
}

pub fn solve_part2() {
    let input = get_input();
    let almanax = parse_input(input);
    let solution = get_solution_part2(almanax);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn it_solves_example_part1() {
        let almanax = parse_input(INPUT);
        let solution = get_solution_part1(almanax);

        assert_eq!(35, solution);
    }

    #[test]
    fn it_solves_example_part2() {
        let almanax = parse_input(INPUT);
        let solution = get_solution_part2(almanax);

        assert_eq!(46, solution);
    }
}
