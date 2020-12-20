use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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

    let mut all_ranges: BTreeSet<usize> = BTreeSet::new();
    let mut rules = HashMap::new();
    for line in handle.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let (name, range1, range2) = parse_rule(&line);
        let mut range = BTreeSet::new();
        range.extend(range1);
        range.extend(range2);
        all_ranges.extend(range.iter());
        rules.insert(name.to_owned(), range);
    }

    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        if line == "your ticket:" {
            break;
        }
    }

    let mut your_ticket_str = String::new();
    stdin.read_line(&mut your_ticket_str).unwrap();
    let your_ticket: Vec<usize> = your_ticket_str
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        if line == "nearby tickets:" {
            break;
        }
    }

    let mut possible = HashMap::new();

    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split(',').map(|n| n.parse().unwrap()).collect();

        if fields.iter().any(|n| !all_ranges.contains(n)) {
            continue;
        }

        if possible.len() != fields.len() {
            let set: HashSet<&str> = rules.keys().fold(HashSet::new(), |mut acc, b| {
                acc.insert(b);
                acc
            });

            for i in 0..fields.len() {
                possible.insert(i, set.clone());
            }
        }

        for (i, value) in fields.iter().enumerate() {
            possible
                .get_mut(&i)
                .unwrap()
                .retain(|&name| rules[name].contains(value));
        }
    }

    let mut result = BTreeMap::new();

    while let Some((id, certain)) = possible
        .iter()
        .find(|(_, set)| set.len() == 1)
        .map(|(&id, set)| (id, set.iter().next().unwrap().to_owned()))
    {
        result.insert(id, certain);

        for set in possible.values_mut() {
            set.remove(certain);
        }
    }

    let answer = result
        .iter()
        .filter(|(_, field)| field.starts_with("departure"))
        .fold(1, |acc, (&idx, _)| acc * your_ticket[idx]);
    println!("{:?}", answer);
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
