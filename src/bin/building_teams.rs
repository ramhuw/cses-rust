use std::{collections::VecDeque, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().map(|line| line.split_whitespace().map(|a| a.parse::<usize>().unwrap()));
    let mut nm = lines.next().unwrap();
    let n = nm.next().unwrap();
    let mut people: Vec<Vec<usize>> = vec![vec![]; n];
    while let Some(mut line) = lines.next() {
        let a = line.next().unwrap() - 1;
        let b = line.next().unwrap() - 1;
        people[a].push(b);
        people[b].push(a);
    }
    let mut assigns: Vec<Option<bool>> = vec![None; n];
    for i in 0..n {
        if assigns[i].is_none() {
            let mut waitlist = VecDeque::from([(i, false)]);
            while let Some((person, assign)) = waitlist.pop_back() {
                if assigns[person] == Some(!assign) {
                    println!("IMPOSSIBLE");
                    return;
                }
                if assigns[person].is_none() {
                    assigns[person] = Some(assign);
                    for &friend in &people[person] {
                        waitlist.push_front((friend, !assign));
                    }
                }
            }

        } 
    }
    println!("{}", assigns.iter().map(|x| if x == &Some(false) {"1"} else {"2"}).collect::<Vec<&str>>().join(" "));
}