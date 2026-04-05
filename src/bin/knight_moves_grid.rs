use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");
    let mut result = vec![vec![-1i32; n]; n];

    let moves: [(i32, i32); 8] = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];

    let mut queue = VecDeque::new();
    result[0][0] = 0;
    queue.push_back((0usize, 0usize));

    while let Some((x, y)) = queue.pop_front() {
        let step = result[x][y];
        for &(dx, dy) in &moves {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if result[nx][ny] == -1 {
                    result[nx][ny] = step + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    let mut out = String::new();
    for line in &result {
        for (i, v) in line.iter().enumerate() {
            if i > 0 { out.push(' '); }
            out.push_str(&v.to_string());
        }
        out.push('\n');
    }
    print!("{}", out);
}
