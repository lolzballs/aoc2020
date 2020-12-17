use std::collections::BTreeMap;

// const TARGET: usize = 2020;
const TARGET: usize = 30000000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut last = None;
    let mut game = BTreeMap::new();
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .enumerate()
        .for_each(|(round, num)| {
            last = game.insert(num, round + 1).map(|r| (num, r));
        });

    for round in *game.values().max().unwrap()..TARGET {
        let num = match last {
            Some((num, r)) => game[&num] - r,
            None => 0,
        };
        last = game.insert(num, round + 1).map(|r| (num, r));
        if round + 1 == TARGET {
            println!("{} {}", round + 1, num);
        }
    }
}
