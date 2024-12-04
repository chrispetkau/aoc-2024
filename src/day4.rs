use aoc_util::grid::{direction::Direction, Grid};
use enum_iterator::all;

type Cell = char;

#[aoc_generator(day4)]
pub fn transform_input(input: &str) -> Grid<Cell> {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();
    Grid::new(width, cells)
}

#[aoc(day4, part1)]
pub fn part1(input: &Grid<Cell>) -> usize {
    // Iterate through all cells. For each 'X' found, travel in each of 8 cardinal directions looking for 'MAS'.
    input
        .indices()
        .map(|i| {
            if input.cells()[i] == 'X' {
                all::<Direction>()
                    .filter(|direction| {
                        if let Ok(ray) = input.ray_from_index(i, *direction) {
                            let mut candidate = ray.skip(1).take(3);
                            candidate.next().filter(|m| **m == 'M').is_some()
                                && candidate.next().filter(|a| **a == 'A').is_some()
                                && candidate.next().filter(|s| **s == 'S').is_some()
                        } else {
                            false
                        }
                    })
                    .count()
            } else {
                0
            }
        })
        .sum::<usize>()
}

#[aoc(day4, part2)]
pub fn part2(input: &Grid<Cell>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day4.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 18);
        // assert_eq!(super::part1(&read_input()), 166357705);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 18);
        // assert_eq!(super::part2(&read_input()), 88811886);
    }
}
