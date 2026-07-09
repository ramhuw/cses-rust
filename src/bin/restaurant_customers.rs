use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut aa: Vec<u32> = Vec::new();
    let mut bb: Vec<u32> = Vec::new();
    while let Some(line) = lines.next() {
        let mut ab = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
        let a = ab.next().unwrap();
        aa.push(a);
        let b = ab.next().unwrap();
        bb.push(b);
    }
    aa.sort();
    bb.sort();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut record: usize = 0;
    let mut maximum: usize = 0;
    while i < n || j < n {
        if i < n && j < n && aa[i] == bb[j] {
            i += 1;
            j += 1;
        } else if i < n && (j < n && aa[i] < bb[j] ||  j >= n) {
            i += 1;
            record += 1;
        } else {
            j += 1;
            record -= 1;
        }
        maximum = maximum.max(record);
    }
    println!("{maximum}");
}
