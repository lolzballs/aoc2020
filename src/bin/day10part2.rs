use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let mut inputs: Vec<usize> = handle
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    inputs.sort();
    inputs.insert(0, 0);

    let mut dp = vec![0u64; *inputs.iter().max().unwrap() + 4];
    dp[0] = 1;
    for i in inputs {
        dp[i + 1] += dp[i];
        dp[i + 2] += dp[i];
        dp[i + 3] += dp[i];
    }

    println!("{}", dp.iter().last().unwrap());
}
