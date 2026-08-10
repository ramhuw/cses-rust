use std::{io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        grid.push(tokens.next().unwrap().chars().collect());
    }
    let mut searches = vec![(0usize, 0usize)];
    let mut alive: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut ans = String::new();
    for l in 0..(n * 2 - 1) {
        let mut c = 'Z';
        for &(i, j) in &searches {
            alive[i][j] = true;
            if grid[i][j] < c {
                c = grid[i][j];
            }
        }
        ans.push(c);
        if l == n * 2 - 2 {
            break;
        }
        let mut new_searches = vec![];
        let mut x = if (l + 1) > n - 1 {n-1} else {l+1};
        let start = x;
        let mut y = if (l + 1) > n - 1 {l + 2 - n} else {0};
        loop {
            if x > 0 && alive[x-1][y] && grid[x-1][y] == c || y > 0 && alive[x][y-1] && grid[x][y-1] == c {
                new_searches.push((x, y));
            }
            if y == start {
                break;
            }
            x -= 1;
            y += 1;
        }
        searches = new_searches;
    }
    println!("{ans}");
}