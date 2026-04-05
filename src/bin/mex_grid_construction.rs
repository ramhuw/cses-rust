use std::collections::BTreeSet;
use std::io::stdin;

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut grid: Vec<Vec<u32>> = vec![vec![0u32; n]; n];
    if n == 1 {
        println!("0");
    } else {
        let mut sets: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); n];
        for i in 0..n {
            for j in 0..=i {
                let mut counter: u32 = 0;
                let search_set = sets[j].union(&sets[i]);
                for &value in search_set {
                    if value == counter {
                        counter += 1;
                    } else {
                        break;
                    }
                }
                sets[i].insert(counter);
                sets[j].insert(counter);
                grid[i][j] = counter;
                grid[j][i] = counter;
            }
        }
        for row in grid {
            println!(
                "{}",
                row.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
