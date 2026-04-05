fn main() {
    let mut input: Vec<String> = Vec::new();
    for _ in 0..8 {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        input.push(line.trim().to_string());
    }
    let mut board: [[bool; 8]; 8] = [[true; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if input[i].chars().nth(j).unwrap() == '*' {
                board[i][j] = false;
            }
        }
    }
    let mut count: i32 = 0;
    let mut queen_count = 0;
    solve(&board, &mut count, &mut queen_count);
    println!("{}", count);
}

fn solve(board: &[[bool; 8]; 8], count: &mut i32, queen_count: &mut i32) {
    let i = *queen_count as usize;
        for j in 0..8 {
            if board[i][j] {
                let mut new_board = board.clone();
                for k in 0..8 {
                    new_board[i][k] = false;
                    new_board[k][j] = false;
                }
                for k in 0..8 {
                    if i + k < 8 && j + k < 8 {
                        new_board[i + k][j + k] = false;
                    }
                    if i + k < 8 && j as i32 - k as i32 >= 0 {
                        new_board[i + k][j - k] = false;
                    }
                    if i as i32 - k as i32 >= 0 && j + k < 8 {
                        new_board[i - k][j + k] = false;
                    }
                    if i as i32 - k as i32 >= 0 && j as i32 - k as i32 >= 0 {
                        new_board[i - k][j - k] = false;
                    }
                }
                let mut new_queen_count = *queen_count + 1;
                if new_queen_count < 8 {
                solve(&new_board, count, &mut new_queen_count);
                } else {
                    *count += 1;
                }
            }
        }
}
