use std::{collections::HashSet, fmt::Write, io::Read};


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse::<usize>().unwrap();
    let m: usize = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut monster_grid: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut monsters: Vec<(usize, usize)> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let mut char_grid: Vec<Vec<bool>> = vec![vec![false; m];n];
    let mut chara_grid: Vec<Vec<Option<char>>> = vec![vec![None; m]; n];
    let mut charas: Vec<(usize, usize, Option<char>)> = vec![];
    let mut grid: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut exits: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n {
        let mut line = tokens.next().unwrap().chars();
        for j in 0..m {
            let c = line.next().unwrap_or('#');
            if c != 'M' && c != '#' && (i == 0 || j == 0 || i == n - 1 || j == m - 1) {
                exits.insert((i, j));
            }
            if c == '#' {
                grid[i][j] = true;
            }
            if c == 'A' {
                charas.push((i, j, None));
                start = (i, j);
            }
            if c == 'M' {
                monsters.push((i, j));
            }
        }
    }
    while !exits.is_empty() && !charas.is_empty() {
        let mut new_monsters = vec![];
        for &(i, j) in &monsters {
            if !grid[i][j] && !monster_grid[i][j] {
                monster_grid[i][j] = true;
                if exits.contains(&(i, j)) {
                    exits.remove(&(i, j));
                }
                if i > 0 {
                    new_monsters.push((i-1, j));
                }
                if i < n - 1 {
                    new_monsters.push((i+1, j));
                }
                if j > 0 {
                    new_monsters.push((i, j-1));
                }
                if j < m - 1 {
                    new_monsters.push((i, j+1));
                }
            }
        }
        monsters = new_monsters;
        let mut new_charas = vec![];
        for &(i, j, last) in &charas {
            if !monster_grid[i][j] && !grid[i][j] && !char_grid[i][j] {
                char_grid[i][j] = true;
                chara_grid[i][j] = last;
                if exits.contains(&(i, j)) {
                    let mut ans = "YES\n".to_string();
                    let mut path: String = String::new();
                    let mut count: usize = 0;
                    let (mut k, mut l) = (i, j);
                    while (k, l) != start {
                        count += 1;
                        let c = chara_grid[k][l].unwrap();
                        path.push(c);
                        match c {
                            'L' => l += 1,
                            'R' => l -= 1,
                            'U' => k += 1,
                            'D' => k -= 1,
                            _ => {}
                        }
                    }
                    write!(ans, "{}\n", count).unwrap();
                    write!(ans, "{}", path.chars().rev().collect::<String>()).unwrap();
                    println!("{ans}");
                    return;
                }
                if i > 0 {
                    new_charas.push((i-1, j, Some('U')));
                }
                if i < n - 1 {
                    new_charas.push((i+1, j, Some('D')));
                }
                if j > 0 {
                    new_charas.push((i, j-1, Some('L')));
                }
                if j < m - 1 {
                    new_charas.push((i, j+1, Some('R')));
                }
            }
        }
        charas = new_charas;
    }
    println!("NO");
}