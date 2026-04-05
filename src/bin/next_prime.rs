fn main() {
    let bases: Vec<u128> = vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    let small_primes: Vec<u128> = vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let n: u128 = input.trim().parse().unwrap();
        if n == 1 {
            println!("2");
            continue;
        }
        let mut c = match n % 2 {
            1 => n + 2,
            _ => n + 1,
        };
        'ulter: loop {
            if small_primes.contains(&c) {
                println!("{}", c);
                break;
            }
            for &p in &small_primes {
                if c % p == 0 {
                    c += 2;
                    continue 'ulter;
                }
            }
            let mut d = c - 1;
            let mut s = 0;
            while d % 2 == 0 {
                d /= 2;
                s += 1;
            }
            let mut adics: Vec<u128> = Vec::new();
            let mut dd = d;
            while dd != 0 {
                adics.push(dd % 2);
                dd /= 2;
            }
            'outer: for &a in &bases {
                let mut x = fast_power(a, d, &adics, c);
                if x == 1 || x == c - 1 {
                    continue;
                } else {
                    for _ in 0..(s - 1) {
                        x = (x * x) % c;
                        if x == c - 1 {
                            continue 'outer;
                        } else if x == 1 {
                            c += 2;
                            continue 'ulter;
                        }
                    }
                    c += 2;
                    continue 'ulter;
                }
            }
            println!("{}", c);
            break;
        }
    }
}

fn fast_power(a: u128, d: u128, adics: &Vec<u128>, c: u128) -> u128 {
    let mut powers: Vec<u128> = Vec::new();
    powers.push(a);
    let mut pow = 1;
    let mut i = 1;
    while 1 << i <= d {
        powers.push((powers[i - 1] * powers[i - 1]) % c);
        i += 1;
    }
    for (index, &adic) in adics.iter().enumerate() {
        if adic == 1 {
            pow = (pow * powers[index]) % c;
        }
    }
    pow
}
