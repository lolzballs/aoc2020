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

    let target = inputs
        .windows(26)
        .find(|&w| !has_pair(w[..25].iter().copied().collect(), w[25]))
        .unwrap()[25];

    println!("target {}", target);

    let mut prefix = inputs.clone();
    for i in 1..prefix.len() {
        prefix[i] += prefix[i - 1];
        for j in 0..i {
            if prefix[i] - prefix[j] == target {
                println!(
                    "{} + {}",
                    inputs[j + 1..i + 1].iter().min().unwrap(),
                    inputs[j + 1..i + 1].iter().max().unwrap(),
                );
            }
        }
    }
}
