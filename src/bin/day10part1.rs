use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let mut inputs: Vec<i64> = handle
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    inputs.sort();
    inputs.insert(0, 0);

    let (ones, twos, threes) = inputs
        .windows(2)
        .fold((0, 0, 0), |(ones, twos, threes), a| match a[1] - a[0] {
            1 => (ones + 1, twos, threes),
            2 => (ones, twos + 1, threes),
            3 => (ones, twos, threes + 1),
            _ => panic!(),
        });

    println!("{} {}", ones, threes + 1);
}
