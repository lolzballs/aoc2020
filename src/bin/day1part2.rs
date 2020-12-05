use std::io::{self, BufRead};

use itertools::iproduct;

fn main() {
    let stdin = io::stdin();
    let values: Vec<u64> = {
        let handle = stdin.lock();
        handle
            .lines()
            .map(|f| f.unwrap().parse().unwrap())
            .collect()
    };

    let result = iproduct!(values.iter(), values.iter(), values.iter())
        .filter(|(&a, &b, &c)| a + b + c == 2020)
        .take(1)
        .map(|(a, b, c)| a * b * c)
        .next();

    println!("{:?}", result);
}
