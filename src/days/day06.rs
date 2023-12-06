struct Record {
    time: usize,
    distance: usize,
}

impl Record {
    fn compute_winning_starting_times(&self) -> usize {
        (0..self.time)
            .filter(|holding_time| {
                let starting_speed = holding_time;
                let remaining_time = self.time - holding_time;
                let reached_distance = remaining_time * starting_speed;

                reached_distance > self.distance
            })
            .count()
    }
}

fn get_input() -> &'static str {
    include_str!("../../data/day06.txt")
}

fn parse_input(input: &str) -> Vec<Record> {
    let records = input
        .split('\n')
        .map(|s| s.split_once(':').unwrap().1)
        .take(2)
        .collect::<Vec<_>>();

    records[0]
        .split_whitespace()
        .zip(records[1].split_whitespace())
        .map(|(time, distance)| Record {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect()
}

fn parse_fixed_input(input: &str) -> Record {
    let records = input
        .split('\n')
        .map(|s| s.split_once(':').unwrap().1)
        .take(2)
        .collect::<Vec<_>>();

    Record {
        time: records[0].split_whitespace().collect::<String>().parse().unwrap(),
        distance: records[1].split_whitespace().collect::<String>().parse().unwrap(),
    }
}

fn get_solution_part1(records: Vec<Record>) -> usize {
    records.iter().map(Record::compute_winning_starting_times).product()
}

fn get_solution_part2(record: Record) -> usize {
    record.compute_winning_starting_times()
}

pub fn solve_part1() {
    let input = get_input();
    let records = parse_input(input);
    let solution = get_solution_part1(records);

    println!("{solution}");
}

pub fn solve_part2() {
    let input = get_input();
    let record = parse_fixed_input(input);
    let solution = get_solution_part2(record);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_solves_example_part1() {
        let records = parse_input(INPUT);
        let solution = get_solution_part1(records);

        assert_eq!(288, solution);
    }

    #[test]
    fn it_solves_example_part2() {
        let record = parse_fixed_input(INPUT);
        let solution = get_solution_part2(record);

        assert_eq!(71503, solution);

    }
}
