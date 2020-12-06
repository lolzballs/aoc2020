use std::collections::HashMap;
use std::io::Read;

type Validator = fn(&str) -> bool;

fn byr_valid(value: &str) -> bool {
    value.parse().map_or(false, |v| 1920 <= v && v <= 2002)
}

fn iyr_valid(value: &str) -> bool {
    value.parse().map_or(false, |v| 2010 <= v && v <= 2020)
}

fn eyr_valid(value: &str) -> bool {
    value.parse().map_or(false, |v| 2020 <= v && v <= 2030)
}

fn hgt_valid(value: &str) -> bool {
    let suffix = &value[value.len() - 2..];
    let value = &value[..value.len() - 2];
    match suffix {
        "cm" => value.parse().map_or(false, |v| 150 <= v && v <= 193),
        "in" => value.parse().map_or(false, |v| 59 <= v && v <= 76),
        _ => false,
    }
}

fn hcl_valid(value: &str) -> bool {
    use regex::Regex;
    let re = Regex::new("^#([a-f]|[0-9]){6}$").unwrap();
    re.is_match(value)
}

fn ecl_valid(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn pid_valid(value: &str) -> bool {
    use regex::Regex;
    let re = Regex::new("^([0-9]){9}$").unwrap();
    re.is_match(value)
}

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();

    let mandatory: HashMap<&str, Validator> = [
        ("byr", byr_valid as Validator),
        ("iyr", iyr_valid as Validator),
        ("eyr", eyr_valid as Validator),
        ("hgt", hgt_valid as Validator),
        ("hcl", hcl_valid as Validator),
        ("ecl", ecl_valid as Validator),
        ("pid", pid_valid as Validator),
    ]
    .iter()
    .cloned()
    .collect();

    let count: usize = input
        .split("\n\n")
        .map(|p| p.replace('\n', " "))
        .filter(|p| {
            p.split_whitespace()
                .map(|f| (&f[..3], &f[4..]))
                .filter(|(k, _)| *k != "cid")
                .filter(|(k, v)| mandatory[k](v))
                .count()
                == 7
        })
        .count();

    println!("{}", count);
}
