use std::collections::BTreeSet;
use std::io::BufRead;

fn has_pair(set: BTreeSet<i64>, target: i64) -> bool {
    for a in &set {
        if set.contains(&(target - a)) {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let inputs: Vec<_> = handle
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let window = inputs
        .windows(26)
        .find(|&w| !has_pair(w[..25].iter().copied().collect(), w[25]));

    println!("{:?}", window.unwrap()[25]);
}
