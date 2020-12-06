use std::collections::HashMap;
use std::io::Read;
use std::iter::Iterator;

fn split_groups(input: &str) -> impl Iterator<Item = &str> {
    input.trim_end().split("\n\n")
}

fn count_questions(group: &str) -> usize {
    let chars = group.chars().fold(HashMap::new(), |mut set, c| {
        let count = set.entry(c).or_insert(0);
        *count += 1;
        set
    });

    let members = chars.get(&'\n').unwrap_or(&0) + 1;

    chars
        .iter()
        .filter(|(k, &v)| !k.is_whitespace() && v == members)
        .count()
}

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();

    let total: usize = split_groups(&input).map(count_questions).sum();
    println!("{}", total);
}
