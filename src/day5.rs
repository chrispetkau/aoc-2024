use std::collections::{HashMap, HashSet};

type PageNumber = usize;

type Update = Vec<PageNumber>;

#[derive(Debug)]
pub struct Input {
    /// Associate with each page number a set of page numbers that must precede it.
    page_ordering_rules: HashMap<PageNumber, HashSet<PageNumber>>,
    updates: Vec<Update>,
}

#[aoc_generator(day5)]
pub fn transform_input(input: &str) -> Input {
    let mut page_ordering_rules: HashMap<PageNumber, HashSet<PageNumber>> = HashMap::new();
    let mut updates = vec![];
    let mut section = 0;
    for line in input.lines() {
        if line.is_empty() {
            section += 1;
            continue;
        }
        match section {
            0 => {
                let mut page_numbers = line.split('|');
                let a = page_numbers.next().unwrap().parse::<usize>().unwrap();
                let b = page_numbers.next().unwrap().parse::<usize>().unwrap();
                page_ordering_rules.entry(b).or_default().insert(a);
            }
            1 => {
                updates.push(
                    line.split(',')
                        .map(|page_number| page_number.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
            _ => unreachable!(),
        }
    }
    Input {
        page_ordering_rules,
        updates,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .updates
        .iter()
        .filter(|update| {
            for i in 0..update.len() - 1 {
                let Some(predecessors) = input.page_ordering_rules.get(&update[i]) else {
                    continue;
                };
                if update
                    .iter()
                    .skip(i)
                    .any(|successor| predecessors.contains(successor))
                {
                    return false;
                }
            }
            true
        })
        .map(|ordered_update| ordered_update[ordered_update.len() / 2])
        .sum::<usize>()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day5.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 143);
        assert_eq!(super::part1(&super::transform_input(&read_input())), 4959);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 9);
        // assert_eq!(super::part2(&super::transform_input(&read_input())), 1866);
    }
}
