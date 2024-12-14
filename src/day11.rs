use aoc_util::grid::coord::Coord;
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day11)]
pub fn transform_input(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn blink_stone(stone: usize) -> Vec<usize> {
    let mut output = vec![];
    if stone == 0 {
        output.push(1);
    } else {
        let digit_count = stone.checked_ilog10().unwrap_or(0) + 1;
        if digit_count % 2 == 0 {
            let s = stone.to_string();
            let mid = s.len() / 2;
            output.push(s[..mid].parse::<usize>().unwrap());
            output.push(s[mid..].parse::<usize>().unwrap());
        } else {
            output.push(stone * 2024);
        }
    }
    output
}

// fn blink(stones: &Vec<usize>) -> Vec<usize> {
//     let mut output = Vec::with_capacity(stones.len() * 2);
//     for i in 0..stones.len() {
//         let stone = &stones[i];
//         if *stone == 0 {
//             output.push(1);
//         } else {
//             let digit_count = stone.checked_ilog10().unwrap_or(0) + 1;
//             if digit_count % 2 == 0 {
//                 let s = stone.to_string();
//                 let mid = s.len() / 2;
//                 output.push(s[..mid].parse::<usize>().unwrap());
//                 output.push(s[mid..].parse::<usize>().unwrap());
//             } else {
//                 output.push(*stone * 2024);
//             }
//         }
//     }
//     output
// }

fn blink_recursive(
    cache: &mut Vec<HashMap<usize, Vec<usize>>>,
    stones: &Vec<usize>,
    blink_count: usize,
) -> Vec<usize> {
    // cache[0].extend(stones);

    // 'open' is the set of pending insertions. Their corresponding cached value may be filled in
    // before they are processed, in which case they can be immediately discarded upon
    // consideration.
    let mut open = (1..=blink_count)
        .flat_map(move |blink_count| stones.iter().map(move |stone| (*stone, blink_count)))
        .collect::<VecDeque<_>>();

    while let Some((stone, blink_count)) = open.pop_front() {
        // Already cached. Move along.
        if cache[blink_count].get(&stone).is_some() {
            continue;
        }

        // Previous blink is in the cache. Add
        if let Some(prior_stones) = cache[blink_count - 1].get(&stone) {
            prior_stones
                .iter()
                .filter(|stone| cache[blink_count].get(&stone).is_none())
                .for_each(|stone| open.push_front((*stone, blink_count)));
        }
        let stones = blink_stone(stone);
    }
    // let mut blinked_stones = vec![];
    // for stone in stones.iter(){
    //     if blink_count == 0 {
    //         blinked_stones.push(*stone);
    //         continue;}
    //         if let Some(blinked_stones_for_stone) = cache[blink_count].get(stone) {
    //             blinked_stones.append(blinked_stones_for_stone);
    //             continue;
    //         }
    //
    // }
    // blinked_stones
    vec![]
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<usize>) -> usize {
    blink_recursive(&mut vec![HashMap::new(); 25], input, 25).len()
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<usize>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "125 17";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day11.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 55312);
        // assert_eq!(super::part1(&super::transform_input(&read_input())), 222);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 34);
        // assert_eq!(super::part2(&super::transform_input(&read_input())), 884);
    }
}
