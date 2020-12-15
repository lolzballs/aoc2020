fn parse_times(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
}

fn main() {
    let mut line_buf = String::new();

    std::io::stdin().read_line(&mut line_buf).unwrap();
    let start_time: i64 = line_buf.trim().parse().unwrap();
    line_buf.clear();

    std::io::stdin().read_line(&mut line_buf).unwrap();
    let bus = parse_times(line_buf.trim())
        .min_by_key(|c| c - start_time % c)
        .unwrap();

    println!("{}", bus * (bus - start_time % bus));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_times("7,13,x,x,59,x,31,19").collect::<Vec<_>>(),
            vec![7, 13, 59, 31, 19]
        );
    }
}
