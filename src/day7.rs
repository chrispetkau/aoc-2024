use enum_iterator::{all, Sequence};
use std::{num::ParseIntError, str::FromStr};

fn generate_permutations<T>(length: usize, elements: Vec<T>) -> Vec<Vec<Vec<T>>>
where
    T: Sized + Copy,
{
    let tier_1_permutations = elements
        .iter()
        .map(|element| vec![*element])
        .collect::<Vec<_>>();
    let mut permutations = vec![vec![], tier_1_permutations];
    for i in 2..length {
        let new_permutations = permutations[i - 1]
            .iter()
            .flat_map(|previous_permutation| {
                elements.iter().map(|element| {
                    let mut new_permutation = previous_permutation.clone();
                    new_permutation.push(*element);
                    new_permutation
                })
            })
            .collect::<Vec<_>>();
        permutations.push(new_permutations);
    }
    permutations
}

#[derive(Clone, Debug)]
pub struct Equation {
    solution: usize,
    terms: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = s.split(':');
        let solution = sides.next().unwrap().parse::<usize>()?;
        let terms = sides
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|term| term.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { solution, terms })
    }
}

impl Equation {
    fn is_solvable<T>(&self, permutations: &Vec<Vec<T>>) -> bool
    where
        T: Op,
    {
        let init = self.terms[0];
        permutations.iter().any(|permutation| {
            let mut op = permutation.iter();
            let candidate = self.terms.iter().skip(1).fold(init, |current, term| {
                op.next().unwrap().apply(current, *term)
            });
            candidate == self.solution
        })
    }
}

#[derive(Clone, Debug)]
pub struct Input(Vec<Equation>);

#[aoc_generator(day7)]
pub fn transform_input(input: &str) -> Input {
    let equations = input
        .lines()
        .map(|line| line.parse::<Equation>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    Input(equations)
}

trait Op {
    fn apply(&self, a: usize, b: usize) -> usize;
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Sequence)]
enum Part1Op {
    Add,
    Multiply,
}

impl Op for Part1Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match *self {
            Part1Op::Add => a + b,
            Part1Op::Multiply => a * b,
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    let max_term_count = input
        .0
        .iter()
        .map(|equation| equation.terms.len())
        .max()
        .unwrap();
    let permutations = generate_permutations(max_term_count, all::<Part1Op>().collect::<Vec<_>>());
    input
        .0
        .iter()
        .filter_map(|equation| {
            equation
                .is_solvable(&permutations[equation.terms.len() - 1])
                .then_some(equation.solution)
        })
        .sum::<usize>()
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Sequence)]
enum Part2Op {
    Add,
    Multiply,
    Concatenate,
}

impl Op for Part2Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match *self {
            Part2Op::Add => a + b,
            Part2Op::Multiply => a * b,
            Part2Op::Concatenate => format!("{a}{b}").parse::<usize>().unwrap(),
        }
    }
}
#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    let max_term_count = input
        .0
        .iter()
        .map(|equation| equation.terms.len())
        .max()
        .unwrap();
    let permutations = generate_permutations(max_term_count, all::<Part2Op>().collect::<Vec<_>>());
    input
        .0
        .iter()
        .filter_map(|equation| {
            equation
                .is_solvable(&permutations[equation.terms.len() - 1])
                .then_some(equation.solution)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day7.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 3749);
        assert_eq!(
            super::part1(&super::transform_input(&read_input())),
            1399219271639
        );
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 11387);
        assert_eq!(
            super::part2(&super::transform_input(&read_input())),
            275791737999003
        );
    }
}
