// #[aoc_generator(day3)]
// pub fn transform_input(input: &str) -> &str {
//     input
// }

use std::mem::MaybeUninit;

use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|mul| {
            let (_, [lhs, rhs]) = mul.extract();
            lhs.parse::<usize>().unwrap() * rhs.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day3.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&INPUT), 161);
        assert_eq!(super::part1(&read_input()), 166357705);
    }

    #[test]
    fn part2() {
        // assert_eq!(super::part2(&super::transform_input(INPUT)), 4);
        // assert_eq!(super::part2(&super::transform_input(&read_input())), 426);
    }
}
