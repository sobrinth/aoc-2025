use good_lp::{IntoAffineExpression, Solution, SolverModel, variables};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn part_1(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|(l, buttons, _)| {
            let goal = l.bytes().map(|c| c == b'#').collect_vec();

            let mut visited = HashSet::new();
            let mut bfs = VecDeque::new();
            // Do a BFS with skipping already visited states
            bfs.push_back((vec![false; goal.len()], 0u64));
            while let Some((state, steps)) = bfs.pop_front() {
                if visited.contains(&state) {
                    continue;
                }
                if state == goal {
                    return steps;
                }
                visited.insert(state.clone());
                for button in buttons {
                    let mut next_state = state.clone();
                    for &toggle in button {
                        next_state[toggle as usize] = !next_state[toggle as usize];
                    }
                    bfs.push_back((next_state, steps + 1));
                }
            }
            unreachable!();
        })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let machines = parse(input)
        .iter()
        .map(|(_, buttons, jolts)| {
            let mut solver_vars = variables!();
            let button_vars = (0..buttons.len())
                .map(|_| solver_vars.add(good_lp::variable().min(0).integer()))
                .collect_vec();

            let mut machine_problem = good_lp::highs(
                solver_vars.minimise(button_vars.iter().sum::<good_lp::Expression>()),
            );
            let mut expressions = vec![0.into_expression(); jolts.len()];
            for i in 0..buttons.len() {
                for &j in &buttons[i] {
                    expressions[j as usize] += button_vars[i];
                }
            }

            for (expr, &j) in expressions.into_iter().zip(jolts) {
                machine_problem.add_constraint(expr.eq(j as f64));
            }

            let solution = machine_problem.solve().unwrap();
            button_vars.iter().map(|&v| solution.value(v)).sum::<f64>() as u64
        })
        .sum();

    machines
}

fn parse(input: &str) -> Vec<(String, Vec<Vec<u64>>, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            // who needs regex anyways... :)
            let (l_str, rest) = l.split_once(' ').unwrap();
            let (b_str, j_str) = rest.rsplit_once(' ').unwrap();

            let lights = l_str[1..l_str.len() - 1].to_string();
            let buttons: Vec<Vec<u64>> = b_str
                .split(' ')
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|w| w.parse().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let jolts: Vec<u64> = j_str[1..j_str.len() - 1]
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect();
            (lights, buttons, jolts)
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

        assert_eq!(part_1(s), 7);
    }
    #[test]
    fn test_part_2() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

        assert_eq!(part_2(s), 33);
    }
}
