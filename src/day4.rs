use aoc_util::grid::{direction::Direction, vector::Vector, Grid};
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
        .coords()
        .map(|coord| {
            if input[&coord] == 'X' {
                all::<Direction>()
                    .filter(|direction| {
                        let mut candidate = input
                            .ray(coord.clone(), *direction)
                            .skip(1)
                            .take(3)
                            .map(|(cell, _)| *cell);
                        candidate.next().is_some_and(|m| m == 'M')
                            && candidate.next().is_some_and(|a| a == 'A')
                            && candidate.next().is_some_and(|s| s == 'S')
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
    input
        .coords()
        .filter(|coord| input[&coord] == 'A')
        .filter_map(|coord| {
            let ne = input
                .offset_coord(&coord, &Vector::from(Direction::Northeast))
                .map(|coord| input[&coord])?;
            let nw = input
                .offset_coord(&coord, &Vector::from(Direction::Northwest))
                .map(|coord| input[&coord])?;
            let se = input
                .offset_coord(&coord, &Vector::from(Direction::Southeast))
                .map(|coord| input[&coord])?;
            let sw = input
                .offset_coord(&coord, &Vector::from(Direction::Southwest))
                .map(|coord| input[&coord])?;
            Some((ne, nw, se, sw))
        })
        .filter(|(ne, nw, se, sw)| {
            ((*ne == *se && *nw == *sw)
                && ((*ne == 'M' && *nw == 'S') || (*ne == 'S' && *nw == 'M')))
                || ((*ne == *nw && *se == *sw)
                    && ((*ne == 'M' && *se == 'S') || (*ne == 'S' && *se == 'M')))
        })
        .count()
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
        assert_eq!(super::part1(&super::transform_input(&read_input())), 2534);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 9);
        assert_eq!(super::part2(&super::transform_input(&read_input())), 1866);
    }
}
