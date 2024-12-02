use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut elements = line
                .split_ascii_whitespace()
                .map(|element| element.parse::<usize>().unwrap());
            (elements.next().unwrap(), elements.next().unwrap())
        })
        .unzip();
    a.sort();
    b.sort();
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut a = Vec::new();
    let mut b = HashMap::new();
    input.lines().for_each(|line| {
        let mut elements = line
            .split_ascii_whitespace()
            .map(|element| element.parse::<usize>().unwrap());
        a.push(elements.next().unwrap());
        b.entry(elements.next().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    a.iter().map(|a| a * b.get(a).unwrap_or(&0)).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day1.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 11);
        assert_eq!(super::part1(&read_input()), 1765812);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 31);
        assert_eq!(super::part2(&read_input()), 20520794);
    }
}
