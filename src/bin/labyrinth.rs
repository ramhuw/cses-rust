use std::{collections::VecDeque, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut line1 = lines.next().unwrap().split_whitespace().map(|a| a.parse::<usize>().unwrap());
    let n = line1.next().unwrap();
    let m = line1.next().unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    while let Some(line) = lines.next() {
        grid.push(line.chars().collect())
    }
    let mut ai: usize = 0;
    let mut aj: usize = 0;
    let mut bi: usize = 0;
    let mut bj: usize = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'A' {
                ai = i;
                aj = j;
            } else if grid[i][j] == 'B' {
                bi = i;
                bj = j;
            }
        }
    }
    let mut steps: Vec<Vec<Option<usize>>> = vec![vec![None; m]; n];
    let mut searches = VecDeque::new();
    searches.push_back((bi, bj, 0usize));
    while let Some((i, j, step)) = searches.pop_back() {
        if grid[i][j] != '#' && (steps[i][j].is_none() || steps[i][j].unwrap() > step) {
            steps[i][j] = Some(step);
            if i == ai && j == aj {
                break;
            }
            if i > 0 {
                searches.push_front((i - 1, j, step + 1));
            }
            if i < n - 1 {
                searches.push_front((i + 1, j, step + 1));
            }
            if j > 0 {
                searches.push_front((i, j - 1, step + 1));
            }
            if j < m - 1 {
                searches.push_front((i, j + 1, step + 1));
            }
        }
    }
    if let Some(step) = steps[ai][aj] {
        let mut ans = String::from("YES\n") + &step.to_string() + "\n";
        let mut i = ai;
        let mut j = aj;
        let mut current_step = 0usize;
        while current_step < step {
            current_step += 1;
            if i > 0 && steps[i-1][j].is_some() && steps[i-1][j].unwrap() + current_step == step {
                i -= 1;
                ans.push('U');
            } else if i < n - 1 && steps[i+1][j].is_some() && steps[i+1][j].unwrap() + current_step == step {
                i += 1;
                ans.push('D');
            } else if j > 0 && steps[i][j-1].is_some() && steps[i][j-1].unwrap() + current_step == step {
                j -= 1;
                ans.push('L');
            } else if j < m - 1 && steps[i][j+1].is_some() && steps[i][j+1].unwrap() + current_step == step {
                j += 1;
                ans.push('R');
            }
        }
        println!("{}", ans);
    } else {
        println!("NO");
    }
}