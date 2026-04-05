use std::io;
fn main() {
    let mut s = String::new();
    io::stdin()
    .read_line(&mut s)
    .expect("Read failed");
    let arr = s.into_bytes();
    let mut current = arr[0];
    let mut count = vec![false; arr.len()];
    let mut counter: usize = 0;
    count[0] = true;
    for i in 1..arr.len() {
        if arr[i] == current {
            counter += 1;
            count[counter] = true;
        } else {
            current = arr[i];
            counter = 0;
        }
        }
    match count.iter().position(|&x| !x) {
            Some(pos) => print!("{} ", pos),
            None => {},
    }
}
