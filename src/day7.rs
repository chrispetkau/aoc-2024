use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Debug)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match *self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }
}

type Permutation = Vec<Op>; // a sequence of ops
type PermutationTier = Vec<Permutation>; // all permutations of a specific length i.e. a tier

fn generate_permutations(length: usize) -> Vec<PermutationTier> {
    let mut permutations = vec![vec![], vec![vec![Op::Add], vec![Op::Multiply]]];
    for i in 2..length {
        let adds = permutations[i - 1].iter().map(|s| {
            let mut adds = s.clone();
            adds.push(Op::Add);
            adds
        });
        let multiplies = permutations[i - 1].iter().map(|s| {
            let mut multiplies = s.clone();
            multiplies.push(Op::Multiply);
            multiplies
        });
        permutations.push(Vec::from_iter(adds.chain(multiplies)));
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
    fn is_solvable(&self, permutations: &Vec<Permutation>) -> bool {
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
pub struct Input {
    equations: Vec<Equation>,
    permutations: Vec<PermutationTier>,
}

#[aoc_generator(day7)]
pub fn transform_input(input: &str) -> Input {
    let equations = input
        .lines()
        .map(|line| line.parse::<Equation>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let max_term_count = equations
        .iter()
        .map(|equation| equation.terms.len())
        .max()
        .unwrap();
    let permutations = generate_permutations(max_term_count);
    Input {
        equations,
        permutations,
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .equations
        .iter()
        .filter_map(|equation| {
            equation
                .is_solvable(&input.permutations[equation.terms.len() - 1])
                .then_some(equation.solution)
        })
        .sum::<usize>()
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    0
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
        assert_eq!(super::part1(&super::transform_input(&read_input())), 1399219271639);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 6);
        // assert_eq!(super::part2(&super::transform_input(&read_input())), 1618);
    }
}
