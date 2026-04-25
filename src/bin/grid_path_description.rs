use std::io;

enum Action {
    Forward(usize, usize),
    Backward(usize, usize),
}

fn main() {
    use Action::*;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut citer = input.trim().chars();
    let mut restrict: [char; 48] = ['?'; 48];
    for i in 0..48 {
        restrict[i] = citer.next().unwrap();
    }
    let mut stack: Vec<Action> = vec![Action::Forward(0, 0)];
    let mut grid: [[bool; 7]; 7] = [[false; 7]; 7];
    let mut ans: usize = 0;
    let mut l: usize = 0;

    while let Some(action) = stack.pop() {
        match action {
            Forward(i, j) => {
                stack.push(Backward(i, j));
                grid[i][j] = true;
                l += 1;
                if l == 49 && i == 6 && j == 0 {
                    ans += 1;
                    continue;
                } else if l == 49 || (i == 6 && j == 0) {
                    continue;
                }

                if !connects(i, j, &grid) {
                    continue;
                }

                let neighbors = get_neighbors(i, j, restrict[l - 1]);

                for (i, j) in &neighbors {
                    if i != &0 || j != &0 {
                        if !grid[*i][*j] {
                            stack.push(Forward(*i, *j));
                        }
                    }
                }
            }
            Backward(i, j) => {
                grid[i][j] = false;
                l -= 1;
            }
        }
    }
    println!("{}", ans);
}

fn get_neighbors(i: usize, j: usize, c: char) -> [(usize, usize); 4] {
    let mut result = [(0, 0); 4];
    let flag = c == '?';
    let lflag = c == 'L' || flag;
    let rflag = c == 'R' || flag;
    let uflag = c == 'U' || flag;
    let dflag = c == 'D' || flag;
    if i > 0 && uflag {
        result[0] = (i - 1, j);
    }
    if i < 6 && dflag {
        result[1] = (i + 1, j);
    }
    if j > 0 && lflag {
        result[2] = (i, j - 1);
    }
    if j < 6 && rflag {
        result[3] = (i, j + 1);
    }
    result
}

fn connects(i: usize, j: usize, grid: &[[bool; 7]; 7]) -> bool {
    let up =
        i == 0 || grid[i - 1][j] || (j > 0 && grid[i - 1][j - 1]) || (j < 6 && grid[i - 1][j + 1]);
    let down =
        i == 6 || grid[i + 1][j] || (j > 0 && grid[i + 1][j - 1]) || (j < 6 && grid[i + 1][j + 1]);
    let left =
        j == 0 || grid[i][j - 1] || (i > 0 && grid[i - 1][j - 1]) || (i < 6 && grid[i + 1][j - 1]);
    let right =
        j == 6 || grid[i][j + 1] || (i > 0 && grid[i - 1][j + 1]) || (i < 6 && grid[i + 1][j + 1]);
    if i > 0 && i < 6 && !grid[i - 1][j] && !grid[i + 1][j] && left && right {
        return false;
    }
    if j > 0 && j < 6 && !grid[i][j - 1] && !grid[i][j + 1] && up && down {
        return false;
    }
    true
}
