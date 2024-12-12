use aoc_util::grid::coord::Coord;
use std::collections::{HashMap, HashSet};

pub struct Input {
    antennas: HashMap<char, Vec<Coord>>,
    width: usize,
    height: usize,
}

#[aoc_generator(day8)]
pub fn transform_input(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    let mut height = 0;
    let mut antennas = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        height += 1;
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                antennas
                    .entry(c)
                    .or_insert_with(|| vec![])
                    .push(Coord::new(x, y));
            });
    });
    Input {
        antennas,
        width,
        height,
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    let mut antinodes = HashSet::new();
    input.antennas.iter().for_each(|(_, locations)| {
        for i in 0..locations.len() {
            let a = &locations[i];
            locations.iter().skip(i + 1).for_each(|b| {
                let delta = b - a;
                if let Some(antinode) = a - &delta {
                    if antinode.x < input.width && antinode.y < input.height {
                        antinodes.insert(antinode);
                    }
                }
                if let Some(antinode) = b + &delta {
                    if antinode.x < input.width && antinode.y < input.height {
                        antinodes.insert(antinode);
                    }
                }
            });
        }
    });
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let mut antinodes = HashSet::new();
    input.antennas.iter().for_each(|(_, locations)| {
        for i in 0..locations.len() {
            let a = &locations[i];
            locations.iter().skip(i + 1).for_each(|b| {
                antinodes.insert(a.clone());
                antinodes.insert(b.clone());
                let step = b - a;
                let mut delta = step.clone();
                loop {
                    let mut done = true;
                    if let Some(antinode) = a - &delta {
                        if antinode.x < input.width && antinode.y < input.height {
                            antinodes.insert(antinode);
                            done = false;
                        }
                    }
                    if let Some(antinode) = b + &delta {
                        if antinode.x < input.width && antinode.y < input.height {
                            antinodes.insert(antinode);
                            done = false;
                        }
                    }
                    if done {
                        break;
                    }
                    delta += &step;
                }
            });
        }
    });
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day8.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 14);
        assert_eq!(super::part1(&super::transform_input(&read_input())), 222);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 34);
        assert_eq!(super::part2(&super::transform_input(&read_input())), 884);
    }
}
