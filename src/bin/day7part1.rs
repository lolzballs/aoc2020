use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::BufRead;

type Content<'a> = (&'a str, usize);

#[derive(Debug, PartialEq)]
struct Bag<'a> {
    color: &'a str,
    contents: Vec<Content<'a>>,
}

impl<'a> TryFrom<&'a str> for Bag<'a> {
    type Error = nom::error::Error<&'a str>;

    fn try_from(rule: &'a str) -> Result<Self, Self::Error> {
        use nom::bytes::complete::{tag, take_until, take_while1};
        use nom::character::complete::char;
        use nom::combinator::opt;
        use nom::multi::separated_list1;
        use nom::sequence::{pair, separated_pair, terminated, tuple};
        use nom::{Finish, Parser};

        let color = take_until(" bags contain ");
        // We either have
        // "no other bags."
        // OR
        // "{} {} bag(s)"...
        let requirements = tag("no other bags.")
            .map(|_| Vec::new())
            .or(separated_list1(
                tag(", "),
                separated_pair(
                    take_while1(|c: char| c.is_digit(10)).map(|s: &str| s.parse().unwrap()),
                    char(' '),
                    terminated(take_until(" bag"), pair(tag(" bag"), opt(tag("s")))),
                )
                .map(|(c, t)| (t, c)),
            ));

        let (_, (color, _, contents)) =
            tuple((color, tag(" bags contain "), requirements))(rule).finish()?;
        Ok(Self { color, contents })
    }
}

fn transpose<'a>(adj: &'a HashMap<&'a str, Vec<Content>>) -> HashMap<&'a str, Vec<&'a str>> {
    adj.iter()
        .fold(HashMap::new(), |mut map, (root, neighbours)| {
            map.entry(root).or_insert(Vec::new());
            for (c, _) in neighbours {
                let vec = map.entry(c).or_insert(Vec::new());
                vec.push(root);
            }
            map
        })
}

fn count<'a>(
    adj: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    root: &str,
) -> usize {
    adj[root]
        .iter()
        .map(|neighbour| {
            if visited.insert(neighbour) {
                1 + count(adj, visited, neighbour)
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let lines: Vec<_> = handle.lines().map(|l| l.unwrap()).collect();
    let adj = lines
        .iter()
        .map(|l| Bag::try_from(l.as_str()).unwrap())
        .fold(HashMap::new(), |mut map, bag| {
            map.insert(bag.color, bag.contents);
            map
        });

    let transpose = transpose(&adj);
    println!("{}", count(&transpose, &mut HashSet::new(), "shiny gold"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_requirements() {
        assert_eq!(
            Bag::try_from("dark olive bags contain 2 muted brown bags, 1 mirrored tomato bag, 4 bright black bags."),
            Ok(Bag {
                color: "dark olive",
                contents: vec![
                    ("muted brown", 2),
                    ("mirrored tomato", 1),
                    ("bright black", 4)
                ]
            })
        );

        assert_eq!(
            Bag::try_from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            Ok(Bag {
                color: "dark orange",
                contents: vec![("bright white", 3), ("muted yellow", 4),]
            })
        );

        assert_eq!(
            Bag::try_from("faded blue bags contain no other bags."),
            Ok(Bag {
                color: "faded blue",
                contents: vec![]
            })
        );
    }

    #[test]
    fn test_transpose() {
        let original = [("a", vec![("b", 1)]), ("b", vec![]), ("c", vec![("b", 1)])]
            .iter()
            .cloned()
            .collect();

        let mut transpose1 = transpose(&original);
        transpose1.iter_mut().for_each(|(_, v)| v.sort());
        assert_eq!(
            transpose1,
            [("b", vec!["a", "c"]), ("a", vec![]), ("c", vec![])]
                .iter()
                .cloned()
                .collect()
        )
    }

    #[test]
    fn test_count() {
        let transpose1 = [
            ("b", vec!["a", "c"]),
            ("a", vec!["d"]),
            ("c", vec![]),
            ("d", vec![]),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(count(&transpose1, &mut HashSet::new(), "b"), 3);
    }
}
