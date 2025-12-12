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

type Buttons = Vec<Vec<u64>>;
type Joltages = Vec<u64>;

struct Mat {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    vars: Vec<usize>,
}

impl Mat {
    const EPSILON: f64 = 1e-9;
    fn from_machine(buttons: &Buttons, joltages: &Joltages) -> Self {
        /*
        Build up a matrix in this format (example input 1):

        B0  B1  B2  B3  B4  B5  J
        0   0   0   0   1   1   3
        0   1   0   0   0   1   5
        0   0   1   1   1   0   4
        1   1   0   1   0   0   7

        After creating this matrix based on the input run, calculate the RREF (row-reduced echelon form)
        Use the matrix afterward to try out combinations of free variables (aka. Button presses) to check for
        valid solutions.
         */
        let rows = joltages.len();
        let cols = buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // add button values
        for (col, button) in buttons.iter().enumerate() {
            for &affected_joltage in button {
                data[affected_joltage as usize][col] = 1.0;
            }
        }

        // add joltage values
        for (row, &joltage) in joltages.iter().enumerate() {
            data[row][cols] = joltage as f64;
        }

        let mut mat = Self {
            data,
            rows,
            cols,
            vars: vec![],
        };

        mat.rref();

        mat
    }

    fn rref(&mut self) {
        // Calculate a row-reduced echelon form of the matrix
        // @see: https://en.wikipedia.org/wiki/Gaussian_elimination

        // init
        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.cols {
            // find the best pivot row for current col
            let (selected_row, selected_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(idx, row)| (idx, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the selected value is zero, then this is a free variable
            if selected_value < Self::EPSILON {
                self.vars.push(col);
                col += 1;
                continue;
            }

            // swap rows
            self.data.swap(pivot, selected_row);

            // Normalize the pivot row
            let p_value = self.data[pivot][col];
            for d in &mut self.data[pivot][col..=self.cols] {
                *d /= p_value;
            }

            // Eliminate the row in all other rows
            for row in 0..self.rows {
                if row != pivot {
                    let factor = self.data[row][col];
                    if factor.abs() > Self::EPSILON {
                        let p_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[row][col..=self.cols]
                            .iter_mut()
                            .zip(&p_row)
                            .for_each(|(v, &p_v)| {
                                *v -= factor * p_v;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Anything remaining is a free variable
        self.vars.extend(col..self.cols);
    }

    // Check if the selected free variables generate a valid solution. If yes, return the button presses.
    fn valid_value(&self, values: &[usize]) -> Option<usize> {
        // how many button presses are there?
        let mut total_presses = values.iter().sum::<usize>();

        // Calculate matrix based on values
        for row in 0..self.rows {
            // Calculate
            let value = self
                .vars
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (idx, &col)| {
                    acc - self.data[row][col] * (values[idx] as f64)
                });

            // the Solution needs to be positive
            if value < -Self::EPSILON {
                return None;
            }
            let rounded = value.round();
            if (value - rounded).abs() > Self::EPSILON {
                return None;
            }

            total_presses += rounded as usize;
        }

        Some(total_presses)
    }
}

fn dfs(mat: &Mat, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // are we done?
    if idx == mat.vars.len() {
        if let Some(total) = mat.valid_value(values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try a different value for the free variables
    let total: usize = values[..idx].iter().sum();
    for sel in 0..max {
        // prune if sel is over the already found min
        if total + sel > *min {
            break;
        }

        values[idx] = sel;
        dfs(mat, idx + 1, values, min, max);
    }
}

pub fn part_2_manual(input: &str) -> usize {
    let machines = parse(input)
        .iter()
        .map(|(_, buttons, joltages)| {
            let mat = Mat::from_machine(buttons, joltages);

            // DFS over the RREF matrix
            let max = *joltages.iter().max().unwrap() + 1;
            let mut min = usize::MAX;
            let mut values = vec![0; mat.vars.len()];

            dfs(&mat, 0, &mut values, &mut min, max as usize);

            min
        })
        .sum::<usize>();

    machines
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

    #[test]
    fn test_part_2_manual() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

        assert_eq!(part_2_manual(s), 33);
    }
}
