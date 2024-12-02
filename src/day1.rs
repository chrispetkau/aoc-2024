#[aoc_generator(day1)]
pub fn transform_input(input: &str) -> (Vec<usize>, Vec<usize>) {
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
    (a, b)
}

#[aoc(day1, part1)]
pub fn part1((a, b): &(Vec<usize>, Vec<usize>)) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>()
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 11);
    }
}
