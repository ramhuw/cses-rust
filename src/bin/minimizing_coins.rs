use std::io::stdin;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    stdin().read_line(&mut input1).unwrap();
    stdin().read_line(&mut input2).unwrap();
    let mut iter1 = input1.trim().split_whitespace();
    let n: usize = iter1.next().unwrap().parse().unwrap();
    let x: usize = iter1.next().unwrap().parse().unwrap();
    let mut c: Vec<usize> = input2
        .trim()
        .split_whitespace()
        .map(|si| si.parse::<usize>().unwrap())
        .collect();
    c.sort();
    c.reverse();
    let mut coins_index: Vec<usize> = vec![0; x + 1];
    let mut coins_prev_index: Vec<usize> = vec![0; x + 1];
    let mut money_spent: usize = 0;
    let mut coin_num: usize = 0;
    let mut ans: usize = x;
    let mut flag = false;
    'outer: loop {
        if coins_index[coin_num] >= n {
            if coin_num == 0 {
                if flag {
                    println!("{}", ans);
                    return;
                }
                println!("-1");
                return;
            }
            coins_index[coin_num] = 0;
            let coin_just_spent = c[coins_prev_index[coin_num - 1]];
            money_spent -= coin_just_spent;
            coins_prev_index[coin_num] = 0;
            coin_num -= 1;
            //println!("-{}, {coin_num}, {money_spent}", coin_just_spent);
            continue;
        }
        let prev_coin = if coin_num == 0 {
            0
        } else {
            coins_prev_index[coin_num - 1]
        };

        if flag
            && coin_num
                + (x - money_spent + c[coins_index[coin_num].max(prev_coin)] - 1)
                    / c[coins_index[coin_num].max(prev_coin)]
                >= ans
        {
            coins_index[coin_num] = n;
            continue;
        }

        if money_spent == x {
            ans = ans.min(coin_num);
            coins_index[coin_num] = n;
            flag = true;
            continue;
        }
        for i in coins_index[coin_num].max(prev_coin)..n {
            let money_to_spend = money_spent + c[i];
            if money_to_spend <= x {
                money_spent = money_to_spend;
                coins_prev_index[coin_num] = i;
                coins_index[coin_num] = i + 1;
                coin_num += 1;
                //println!("{}, {coin_num}, {money_spent}", c[i]);
                continue 'outer;
            }
        }
        coins_index[coin_num] = n;
    }
}
