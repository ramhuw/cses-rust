const CONST: u64 = 10u32.pow(9) as u64 + 7;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().to_string().parse::<u64>().unwrap();
    let mut adics: Vec<usize> = Vec::new();
    let mut m = n as usize;
    while m != 0 {
        adics.push(m % 2);
        m /= 2;
    }
    let matrix: Vec<Vec<u64>> = vec![vec![0, 1], vec![1, 1]];
    let mut powers: Vec<Vec<Vec<u64>>> = Vec::new();
    powers.push(matrix.clone());
    let mut i: usize = 0;
    while 1 << (i + 1) <= n {
        powers.push(matrix_mul(&powers[i], &powers[i]));
        i += 1;
    }
    let mut mat_power: Vec<Vec<u64>> = vec![vec![1, 0], vec![0, 1]];
    for (index, &adic) in adics.iter().enumerate() {
        if adic != 0 {
            mat_power = matrix_mul(&mat_power, &powers[index])
        }
    }
    println!("{}", mat_power[0][1]);
}

fn matrix_mul(a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut c: Vec<Vec<u64>> = vec![vec![0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % CONST;
            }
        }
    }
    c
}
