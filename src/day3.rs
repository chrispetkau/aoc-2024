// #[aoc_generator(day3)]
// pub fn transform_input(input: &str) -> &str {
//     input
// }

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
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    re.captures_iter(input).for_each(|instruction| {
        let full = instruction.get(0).unwrap().as_str();
        if full.starts_with("mul") {
            if enabled {
                let lhs = instruction.get(1).unwrap().as_str();
                let rhs = instruction.get(2).unwrap().as_str();
                sum += lhs.parse::<usize>().unwrap() * rhs.parse::<usize>().unwrap();
            }
        } else if full.starts_with("don") {
            enabled = false;
        } else {
            enabled = true;
        }
    });
    sum
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day3.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(super::part1(&INPUT), 161);
        assert_eq!(super::part1(&read_input()), 166357705);
    }

    #[test]
    fn part2() {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(super::part2(&INPUT), 48);
        assert_eq!(super::part2(&read_input()), 88811886);
    }
}
