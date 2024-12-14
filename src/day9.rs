use std::iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Occupied(usize),
    Unoccupied,
}

#[aoc_generator(day9)]
pub fn transform_input(input: &str) -> Vec<Cell> {
    let mut id = 0;
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let length = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                let block = iter::repeat_n(Cell::Occupied(id), length);
                id += 1;
                block
            } else {
                iter::repeat_n(Cell::Unoccupied, length)
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Cell>) -> usize {
    let mut output = input.clone();
    let free_space = input
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| (*cell == Cell::Unoccupied).then_some(i));
    let move_blocks = input
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, cell)| (matches!(*cell, Cell::Occupied(_))).then_some(i));
    free_space
        .zip(move_blocks)
        .take_while(|(free_index, move_index)| free_index < move_index)
        .for_each(|(free_index, move_index)| output.swap(free_index, move_index));
    output
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            if let Cell::Occupied(file_id) = *cell {
                i * file_id
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Cell>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "2333133121414131402";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day9.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 1928);
        // assert_eq!(super::part1(&super::transform_input(&read_input())), 2534);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 9);
        // assert_eq!(super::part2(&super::transform_input(&read_input())), 1866);
    }
}
