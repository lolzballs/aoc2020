fn parse_times(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input
        .split(',')
        .map(|s| match s {
            "x" => None,
            s => Some(s.parse().unwrap()),
        })
        .enumerate()
        .filter_map(|(i, id)| id.map(|id| ((id - i % id) % id, id)))
}

fn main() {
    let mut line_buf = String::new();

    std::io::stdin().read_line(&mut line_buf).unwrap();
    let mut constraints: Vec<_> = parse_times(line_buf.trim()).collect();
    constraints.sort_by_key(|(_, modulus)| *modulus);
    println!("{:?}", constraints);

    let min = constraints.remove(0);

    let sol = constraints
        .into_iter()
        .fold(min, |(cur, inc), (rem, modulus)| {
            println!("{} {} (x === {}) mod {}", cur, inc, rem, modulus);
            let mut cur = cur;
            while cur % modulus != rem {
                cur += inc;
            }
            (cur, inc * modulus)
        });
    println!("{}", sol.0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_times("7,13,x,x,59,x,31,19").collect::<Vec<_>>(),
            vec![(0, 7), (12, 13), (55, 59), (25, 31), (12, 19),]
        );
    }
}
