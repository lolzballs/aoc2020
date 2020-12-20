use std::collections::BTreeSet;
use std::io::BufRead;
use std::ops::RangeInclusive;

fn parse_rule<'a>(s: &'a str) -> (&'a str, RangeInclusive<usize>, RangeInclusive<usize>) {
    use regex::Regex;

    let regex = Regex::new(r"^([^:]*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let matches = regex.captures(s).unwrap();

    (
        matches.get(1).unwrap().as_str(),
        matches[2].parse().unwrap()..=matches[3].parse().unwrap(),
        matches[4].parse().unwrap()..=matches[5].parse().unwrap(),
    )
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let mut rules = BTreeSet::new();
    for line in handle.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let (_, range1, range2) = parse_rule(&line);
        rules.extend(range1);
        rules.extend(range2);
    }

    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        if line == "nearby tickets:" {
            break;
        }
    }

    let handle = stdin.lock();
    let count: usize = handle
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .filter(|n| !rules.contains(n))
                .sum::<usize>()
        })
        .sum();

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_rule("departure location: 27-180 or 187-953"),
            ("departure location", 27..=180, 187..=953)
        );
        assert_eq!(
            parse_rule("departure station: 47-527 or 545-958"),
            ("departure station", 47..=527, 545..=958)
        );
    }
}
