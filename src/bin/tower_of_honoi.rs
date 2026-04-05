fn main() {
    use std::io;
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");
    let a = (1 << n) - 1;
    println!("{}", a);
    let v = (0..=a).into_iter().map(|i| {
        i ^ (i >> 1)
    });
    let w = v.clone()
        .skip(1);
    let x = v.zip(w)
        .map(|(a, b): (u32, u32)| {
            (a ^ b).trailing_zeros()
        });
    let mut hanoi = Hanoi::new(n as u32);
    x.for_each(|i| {
        hanoi.move_disk(i)
    });

}

struct Disk {
    label: u32,
    peg: u32,
    heading: i32,
}

impl Disk {
    fn new(label: u32) -> Self {
        Self { label, peg: 0u32, heading: 0i32 }
    }
    fn label(&self) -> u32 {
        self.label
    }
    fn peg(&self) -> u32 {
        self.peg
    }
    fn is_least(&self) -> bool {
        self.label == 0
    }
    fn move_left(&mut self) {
        self.peg = (self.peg + 2) % 3
    }
    fn move_right(&mut self) {
        self.peg = (self.peg + 1) % 3
    }
}

struct Hanoi {
    n: u32,
    pegs: [Vec<Disk>; 3],
    disk_pos: Vec<u32>,
}

impl Hanoi {
    fn new(n: u32) -> Self {
        let mut disks_0: Vec<Disk> = (0..n)
        .rev()
        .into_iter()
        .map(Disk::new)
        .collect();
        disks_0.last_mut().unwrap().heading = if n % 2 == 0 { 1 } else { -1 };
        Self { n, pegs: [disks_0, Vec::new(), Vec::new()], disk_pos: vec![0; n as usize] }
    }
    fn move_disk(&mut self, disk_label:u32)  {
        let mut disk = self.pegs[self.disk_pos[disk_label as usize] as usize]
            .pop()
            .expect("Disk not found");
        let f = disk.peg();
        match disk_label {
            0 => {
                match disk.heading {
                    1 => disk.move_right(),
                    -1 => disk.move_left(),
                    _ => panic!("Invalid heading"),
                }
            },
            _ => {
                let i = self.disk_pos[0];
                match i as i32 - f as i32 {
                    x if x == 1 || x == -2 => disk.move_left(),
                    x if x == -1 || x == 2 => disk.move_right(),
                    _ => panic!("Invalid peg difference"),
                } 
            }
        }
        let t: u32 = disk.peg();
        self.pegs[t as usize].push(disk);
        self.disk_pos[disk_label as usize] = t;
        println!("{} {}", f + 1, t + 1);
    }
}