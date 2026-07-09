use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let xx: Vec<i64> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut sum: i64 = 0;
    let mut sums: Vec<i64> = vec![0];
    let mut min = 0;
    let mut mins: Vec<i64> = vec![0];
    for x in xx {
        sum += x;
        sums.push(sum);
        min = min.min(sum);
        mins.push(min);
    }
    let mut ans = i64::MIN;
    for i in 0..n {
        ans = ans.max(sums[i+1] - mins[i]);
    }
    println!("{ans}");
}