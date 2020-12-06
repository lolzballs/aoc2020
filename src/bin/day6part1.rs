use std::collections::HashSet;
use std::io::Read;
use std::iter::Iterator;

fn split_groups(input: &str) -> impl Iterator<Item = &str> {
    input.split("\n\n")
}

fn count_questions(group: &str) -> usize {
    group
        .chars()
        .filter(|&c| !c.is_whitespace())
        .fold(HashSet::new(), |mut set, c| {
            set.insert(c);
            set
        })
        .len()
}

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();

    let total: usize = split_groups(&input).map(count_questions).sum();
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_count() {
        assert_eq!(
            split_groups(
                r##"
abc

a
b
c

ab
ac

a
a
a
a

b
        "##
            )
            .count(),
            5
        );
    }

    #[test]
    fn count_one_group() {
        assert_eq!(
            count_questions(
                r##"
abcx
abcy
abcz
        "##
            ),
            6
        )
    }
}
