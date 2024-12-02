use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn transform_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|element| element.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn part1(reports: &Vec<Vec<isize>>) -> usize {
    let derivatives: Vec<Vec<isize>> = reports
        .iter()
        .map(|report| {
            report
                .iter()
                .zip(report.iter().skip(1))
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    derivatives
        .iter()
        .filter(|derivative| derivative.iter().all(|delta| delta.abs() >= 1))
        .filter(|derivative| derivative.iter().all(|delta| delta.abs() <= 3))
        .filter(|derivative| {
            derivative.iter().all(|delta| delta.signum() == 1)
                || derivative.iter().all(|delta| delta.signum() == -1)
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(reports: &Vec<Vec<isize>>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day2.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 2);
        assert_eq!(super::part1(&super::transform_input(&read_input())), 371);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 31);
        // assert_eq!(super::part2(&read_input()), 20520794);
    }
}
