use bitvec::bitvec;

use crate::{LineMask, Rule, Rules, Run};

impl Rule {
    pub fn generate_constraints(&mut self) {
        let line_len = self.line_len as usize;

        for color in self.fills.iter_colors() {
            let runs: Vec<_> = self.iter_fill_runs(color).copied().collect();

            let left = fit_forwards(&runs, line_len);
            let right = fit_backwards(&runs, line_len);

            let filled = self.find_filled(&left, &right);

            self.constraints.insert(color, filled);
        }
    }

    fn find_filled(&self, _left: &Vec<Vec<bool>>, _right: &Vec<Vec<bool>>) -> LineMask {
        bitvec![0; self.line_len as usize]
    }
}

/// Fit runs in a line going forwards
///
/// * `runs`: Runs to fit in the line
/// * `line_len`: Length of the line to fit runs in
fn fit_forwards(runs: &[Run], line_len: usize) -> Vec<Vec<bool>> {
    let m = runs.len();
    let n = line_len;

    // dp[offset][r]: runs[0..r] fit in cells[0..offset]
    let mut dp = vec![vec![false; n + 1]; m + 1];

    // No runs can always fit
    dp[0][0] = true;

    for offset in 0..=n {
        for r in 0..=m {
            // Cannot fit another run if previous runs already do not fit
            if !dp[r][offset] {
                continue;
            }

            // Option 1: leave cell empty
            if offset < n {
                dp[r][offset + 1] = true;
            }

            // Already considered all runs
            if r >= m {
                continue;
            }

            // Option 2: place run r
            let len = runs[r].count as usize;
            let mut next = offset + len;

            // Leave a gap for adjacent runs of the same fill
            if r + 1 < m && runs[r].fill == runs[r + 1].fill {
                next += 1;
            }

            if next <= n {
                dp[r + 1][next] = true;
            }
        }
    }

    dp
}

/// Fit runs in a line going backwards
///
/// * `runs`: Runs to fit in the line
/// * `line_len`: Length of the line to fit runs in
fn fit_backwards(runs: &[Run], line_len: usize) -> Vec<Vec<bool>> {
    let m = runs.len();
    let n = line_len;

    // dp[r][offset]: runs[r..m] fit in cells[offset..n]
    let mut dp = vec![vec![false; n + 1]; m + 1];

    // No runs can always fit
    dp[m][n] = true;

    for offset in (0..n).rev() {
        for r in 0..=m {
            // Another run fits if previous runs already do not fit
            if dp[r][offset + 1] {
                // Option 1: leave cell empty
                dp[r][offset] = true;
            }

            // Already considered all runs
            if r == m {
                continue;
            }

            // Option 2: place run r
            let len = runs[r].count as usize;
            let mut next = offset + len;

            // Leave a gap for adjacent runs of the same fill
            if r + 1 < m && runs[r].fill == runs[r + 1].fill {
                next += 1;
            }

            if next <= n && dp[r + 1][next] {
                dp[r][next] = true;
            }
        }
    }

    dp
}

impl Rules {
    pub fn generate_masks(&mut self) {
        let rows = self.rows.len();
        let cols = self.cols.len();

        {
            for (r, rule) in self.rows.iter_mut().enumerate() {
                rule.generate_constraints();
                tracing::info!("[{r}/{rows}] Generated row mask");
            }
        }

        for (c, col) in self.cols.iter_mut().enumerate() {
            col.generate_constraints();
            tracing::info!("[{c}/{cols}] Generated col mask");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Fill;
    use rstest::rstest;

    const B: Fill = Fill::Blank;
    const X: Fill = Fill::Cross;
    const C: Fill = Fill::Color(1);
    const C2: Fill = Fill::Color(2);

    fn ft_vec(len: usize, false_until_idx: usize) -> Vec<bool> {
        (0..len).map(|idx| idx >= false_until_idx).collect()
    }

    fn tf_vec(len: usize, false_until_idx: usize) -> Vec<bool> {
        (0..len).map(|idx| idx >= false_until_idx).collect()
    }

    #[rstest]
    #[case::single_run(vec![(C,1)], 1, vec![ft_vec(2, 0), ft_vec(2, 1)])]
    #[case::single_run(vec![(C,1)], 2, vec![ft_vec(3, 0), ft_vec(3, 1)])]
    #[case::single_run(vec![(C,1)], 3, vec![ft_vec(4, 0), ft_vec(4, 1)])]
    #[case::single_run(vec![(C,2)], 2, vec![ft_vec(3, 0), ft_vec(3, 2)])]
    #[case::single_run(vec![(C,2)], 3, vec![ft_vec(4, 0), ft_vec(4, 2)])]
    #[case::single_run(vec![(C,2)], 4, vec![ft_vec(5, 0), ft_vec(5, 2)])]
    #[case::single_run(vec![(C,3)], 3, vec![ft_vec(4, 0), ft_vec(4, 3)])]
    #[case::single_run(vec![(C,3)], 4, vec![ft_vec(5, 0), ft_vec(5, 3)])]
    #[case::single_run(vec![(C,3)], 5, vec![ft_vec(6, 0), ft_vec(6, 3)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 3, vec![ft_vec(4, 0), ft_vec(4, 2), ft_vec(4, 3)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 4, vec![ft_vec(5, 0), ft_vec(5, 2), ft_vec(5, 3)])]
    #[case::multiple_runs(vec![(C,3), (C,2), (C,1)], 8, vec![ft_vec(9, 0), ft_vec(9, 4), ft_vec(9, 7), ft_vec(9, 8)])]
    #[case::mixed_runs(vec![(C,1), (C,1)], 3, vec![ft_vec(4, 0), ft_vec(4, 2), ft_vec(4, 3)])]
    fn test_fit_forwards(
        #[case] runs: Vec<(Fill, u16)>,
        #[case] line_len: usize,
        #[case] expected: Vec<Vec<bool>>,
    ) {
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();
        let dp = fit_forwards(&runs, line_len);

        assert_eq!(dp, expected);
    }

    #[rstest]
    #[case::single_run(vec![(C,1)], 1, vec![ft_vec(2, 1), ft_vec(2, 0)])]
    #[case::single_run(vec![(C,1)], 2, vec![ft_vec(3, 1), ft_vec(3, 0)])]
    #[case::single_run(vec![(C,1)], 3, vec![ft_vec(4, 1), ft_vec(4, 0)])]
    #[case::single_run(vec![(C,2)], 2, vec![ft_vec(3, 2), ft_vec(3, 0)])]
    #[case::single_run(vec![(C,2)], 3, vec![ft_vec(4, 2), ft_vec(4, 0)])]
    #[case::single_run(vec![(C,2)], 4, vec![ft_vec(5, 2), ft_vec(5, 0)])]
    #[case::single_run(vec![(C,3)], 3, vec![ft_vec(4, 3), ft_vec(4, 0)])]
    #[case::single_run(vec![(C,3)], 4, vec![ft_vec(5, 3), ft_vec(5, 0)])]
    #[case::single_run(vec![(C,3)], 5, vec![ft_vec(6, 3), ft_vec(6, 0)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 3, vec![ft_vec(4, 2), ft_vec(4, 1), ft_vec(4, 0)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 4, vec![ft_vec(5, 2), ft_vec(5, 1), ft_vec(5, 0)])]
    // #[case::multiple_runs(vec![(C,3), (C,2), (C,1)], 8, vec![ft_vec(9, 8), ft_vec(9, 7), ft_vec(9, 4), ft_vec(9, 0)])]
    // #[case::mixed_runs(vec![(C,1), (C,1)], 3, vec![ft_vec(4, 0), vec![false, false, true, true], vec![false, false, false, true]])]
    fn test_fit_backwards(
        #[case] runs: Vec<(Fill, u16)>,
        #[case] line_len: usize,
        #[case] expected: Vec<Vec<bool>>,
    ) {
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();
        let dp = fit_backwards(&runs, line_len);

        assert_eq!(dp, expected);
    }
}
