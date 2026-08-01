use std::{cmp::Reverse, collections::BinaryHeap, io::{self, Read, Write}};

#[derive(Clone)]
struct City {
    neighbors: Vec<(usize, usize)>,
}

impl City {
    fn new() -> Self {
        Self {
            neighbors: Vec::new(),
        }
    }
}

struct Map {
    cities: Vec<City>,
}

impl Map {
    fn new(n: usize) -> Self {
        Self {
            cities: vec![City::new(); n],
        }
    }

    fn insert(&mut self, a: usize, b: usize, c: usize) {
        self.cities[a - 1].neighbors.push((b - 1, c));
    }
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut map = Map::new(n);
    for _ in 0..m {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        let c = tokens.next().unwrap();
        map.insert(a, b, c);
    }
    let mut ans: Vec<usize> = vec![usize::MAX; n];
    ans[0] = 0;
    let mut distances: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::from([Reverse((0, 0))]);
    while let Some(Reverse((d, c))) = distances.pop() {
        if d != ans[c] {
            continue;
        }
        for &(neighbor_id, neighbor_distance) in &map.cities[c].neighbors {
            let new_distance = d + neighbor_distance;
            if ans[neighbor_id] > new_distance {
                distances.push(Reverse((new_distance, neighbor_id)));
                ans[neighbor_id] = new_distance;
            }
        }
    }
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for d in ans {
        write!(out, "{d} ").unwrap();
    }
}
