fn make_derivative(report: &Vec<isize>) -> Vec<isize> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect::<Vec<_>>()
}

#[aoc_generator(day2)]
pub fn transform_input(input: &str) -> (Vec<Vec<isize>>, Vec<Vec<isize>>) {
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|element| element.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let derivatives: Vec<Vec<isize>> = reports.iter().map(make_derivative).collect::<Vec<_>>();
    (reports, derivatives)
}

fn match_gradient(derivative: &Vec<isize>) -> (Vec<usize>, Vec<usize>) {
    debug_assert!(
        delta_size_anomalies(derivative).len() == 0,
        "Assert we have already filtered out delta size anomalies"
    );
    let sign = derivative[0].signum();
    let (matches, mismatches): (Vec<usize>, Vec<usize>) =
        (0..derivative.len()).partition(|i| derivative[*i].signum() == sign);
    if matches.len() < mismatches.len() {
        (matches, mismatches)
    } else {
        (mismatches, matches)
    }
}

fn delta_size_anomalies(derivative: &Vec<isize>) -> Vec<usize> {
    derivative
        .iter()
        .enumerate()
        .filter_map(|(i, delta)| {
            let diff = delta.abs();
            (diff < 1 || diff > 3).then_some(i)
        })
        .collect::<Vec<usize>>()
}

fn is_safe(derivative: &Vec<isize>) -> bool {
    delta_size_anomalies(derivative).len() == 0 && match_gradient(derivative).0.len() == 0
}

#[aoc(day2, part1)]
pub fn part1((_, derivatives): &(Vec<Vec<isize>>, Vec<Vec<isize>>)) -> usize {
    derivatives
        .iter()
        .filter(|derivative| is_safe(*derivative))
        .count()
}

#[aoc(day2, part2)]
pub fn part2((reports, derivatives): &(Vec<Vec<isize>>, Vec<Vec<isize>>)) -> usize {
    let indices = (0..derivatives.len()).collect::<Vec<_>>();
    let (safe, not_safe): (Vec<usize>, Vec<usize>) =
        indices.iter().partition(|i| is_safe(&derivatives[**i]));
    let mut dampened_count = 0;
    let mut try_dampening = |report_index, level_index| -> bool {
        let report: &Vec<isize> = &reports[report_index];
        let mut dampened_report = report.clone();
        dampened_report.remove(level_index);
        let dampened_derivative = make_derivative(&dampened_report);
        let safe = is_safe(&dampened_derivative);
        if safe {
            dampened_count += 1;
        }
        safe
    };
    for i in not_safe.iter() {
        let mut dampened = false;
        let derivative = &derivatives[*i];
        let delta_anomalies = delta_size_anomalies(derivative);
        if delta_anomalies.len() == 1 || delta_anomalies.len() == 2 {
            for j in delta_anomalies.iter() {
                // Try removing the level on either side of the anomaly and see if either "dampening" makes it safe.
                dampened = try_dampening(*i, *j) || try_dampening(*i, *j + 1);
                if dampened {
                    break;
                }
            }
        }
        if dampened {
            continue;
        }
        if !delta_anomalies.is_empty() {
            continue;
        }
        let gradient_anomalies = match_gradient(derivative).0;
        if gradient_anomalies.len() == 1 || gradient_anomalies.len() == 2 {
            for j in gradient_anomalies.iter() {
                // Try removing the level on either side of the anomaly and see if either "dampening" makes it safe.
                dampened = try_dampening(*i, *j) || try_dampening(*i, *j + 1);
                if dampened {
                    break;
                }
            }
        }
    }
    safe.len() + dampened_count
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    fn read_input() -> String {
        let mut input = String::new();
        let _ = std::fs::File::open("input\\2024\\day2.txt")
            .unwrap()
            .read_to_string(&mut input);
        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&super::transform_input(INPUT)), 2);
        assert_eq!(super::part1(&super::transform_input(&read_input())), 371);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&super::transform_input(INPUT)), 4);
        assert_eq!(super::part2(&super::transform_input(&read_input())), 426);
    }
}
