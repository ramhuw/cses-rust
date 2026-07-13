use std::io::Read;

struct Counter {
    counter: usize,
    n: usize,
    m: usize,
    grid: Vec<Vec<char>>,
    rooms: Vec<Vec<usize>>
}

impl Counter {
    fn new(n: usize, m: usize, grid: Vec<Vec<char>>) -> Self {
        Self {
            counter: 0,
            n,
            m,
            grid,
            rooms: vec![vec![0; m]; n]
        }
    }
    fn count(&mut self) -> usize {
        for i in 0..self.n {
            for j in 0..self.m {
                if self.grid[i][j] == '.' && self.rooms[i][j] == 0 {
                    self.counter += 1;
                    let mut searches: Vec<(usize, usize)> = vec![(i, j)];
                    while let Some((x, y)) = searches.pop() {
                        if self.grid[x][y] == '.' && self.rooms[x][y] == 0 {
                            self.rooms[x][y] = self.counter;
                            if x > 0 {
                                searches.push((x - 1, y));
                            }
                            if x < self.n - 1 {
                                searches.push((x + 1, y));
                            }
                            if y > 0 {
                                searches.push((x, y - 1));
                            }
                            if y < self.m - 1 {
                                searches.push((x, y + 1));
                            }
                        }
                    }
                }
            }
        }
        self.counter
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let nm: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];
    let mut grid: Vec<Vec<char>> = vec![];
    while let Some(line) = lines.next() {
        grid.push(line.chars().collect());
    }
    let mut counter = Counter::new(n, m, grid);
    println!("{}", counter.count());
}
