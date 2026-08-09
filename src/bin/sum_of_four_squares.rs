use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let t = tokens.next().unwrap();
    let mut squares: Vec<usize> = Vec::new();
    let mut ans: String = String::new();
    for i in 0.. {
        let sq = i * i;
        if sq > 10000000 {
            break;
        }
        squares.push(sq);
    }
    'outer: for _ in 0..t {
        let n = tokens.next().unwrap();
        let mut left = 0;
        let mut right = squares.len() - 1;
        while left < right {
            let middle = (left + right) / 2;
            let middle_square = squares[middle];
            if middle_square == n {
                left = middle;
                break;
            } else if middle_square < n {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }
        let upper_index = left;
        for i in 0..=upper_index {
            let a2 = squares[i];
            if 4 * a2 > n {
                break;
            }
            for j in i..=upper_index {
                let b2 = squares[j];
                if a2 + b2 * 3 > n {
                    break;
                }
                let target = n - a2 - b2;
                let mut k = j;
                let mut l = upper_index;
                while k <= l {
                    if squares[k] * 2 > target {
                        break;
                    }
                    let value = squares[k] + squares[l];
                    if value == target {
                        ans.push_str(&format!("{} {} {} {}\n", i, j, k, l));
                        continue 'outer;
                    } else if value < target {
                        k += 1;
                    } else {
                        l -= 1;
                    }
                }
            }
        }
    }
    println!("{ans}");
}