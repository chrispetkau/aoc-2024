use aoc_util::grid::{coord::Coord, direction::Direction, rotation::Rotation, Grid};

type Cell = bool;

#[derive(Clone, Debug)]
pub struct Guard {
    position: Coord,
    direction: Direction,
}

#[derive(Clone, Debug)]
pub struct Input {
    grid: Grid<Cell>,
    guard: Guard,
}

#[aoc_generator(day6)]
pub fn transform_input(input: &str) -> Input {
    let mut guard = None;
    let mut cells = Vec::with_capacity(input.len());
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => guard = Some((x, y, Direction::North)),
                'v' => guard = Some((x, y, Direction::South)),
                '<' => guard = Some((x, y, Direction::West)),
                '>' => guard = Some((x, y, Direction::East)),
                _ => {}
            }
            cells.push(c == '#');
        }
    }
    let guard = guard.unwrap();
    Input {
        grid: Grid::new(input.lines().next().unwrap().len(), cells),
        guard: Guard {
            position: Coord::new(guard.0, guard.1),
            direction: guard.2,
        },
    }
}

const N: u8 = 1 << 0;
const S: u8 = 1 << 1;
const E: u8 = 1 << 2;
const W: u8 = 1 << 3;

fn visit(coord: &Coord, direction: Direction, visited: &mut Grid<u8>) -> bool {
    let visit = &mut visited[coord];
    let prev = *visit;
    match direction {
        Direction::North => *visit |= N,
        Direction::South => *visit |= S,
        Direction::East => *visit |= E,
        Direction::West => *visit |= W,
        _ => unreachable!(),
    }
    prev != *visit
}

fn simulate(input: &Input) -> (Grid<u8>, bool) {
    let mut visited_cells = Vec::with_capacity(input.grid.cells().len());
    visited_cells.resize(input.grid.cells().len(), 0);
    let mut visited = Grid::new(input.grid.width(), visited_cells);
    let mut guard = input.guard.clone();
    let mut trapped = false;
    loop {
        let ray = input.grid.ray(guard.position.clone(), guard.direction);
        let mut blocked = false;
        for (cell, coord) in ray {
            if *cell {
                blocked = true;
                break;
            }
            if !visit(&coord, guard.direction, &mut visited) {
                trapped = true;
                break;
            }
            guard.position = coord;
        }
        if !blocked {
            break;
        }
        let new_direction = guard.direction.rotate(Rotation::Clockwise90Degrees);
        guard.direction = new_direction;
    }
    (visited, trapped)
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    let (visited, _) = simulate(input);
    visited.cells().iter().filter(|visit| **visit != 0).count()
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let mut scratch = input.clone();
    let start_index = input.grid.index(&input.guard.position).unwrap();
    input
        .grid
        .indices()
        .filter(|i| *i != start_index)
        .filter(|i| !input.grid.cells()[*i])
        .filter(|i| {
            scratch.grid.cells_mut()[*i] = true;
            let (_, trapped) = simulate(&scratch);
            scratch.grid.cells_mut()[*i] = false;
            trapped
        })
        .count()
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day6.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 41);
        assert_eq!(super::part1(&super::transform_input(&read_input())), 4778);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 6);
        assert_eq!(super::part2(&super::transform_input(&read_input())), 1618);
    }
}
