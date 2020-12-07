use std::collections::HashMap;
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

fn count<'a>(
    adj: &HashMap<&str, Vec<Content<'a>>>,
    visited: &mut HashMap<&'a str, usize>,
    root: &str,
) -> usize {
    adj[root]
        .iter()
        .map(|(neighbour, num)| {
            let memoized = visited.get(neighbour).copied();
            num * (match memoized {
                Some(count) => count,
                None => {
                    let count = count(adj, visited, neighbour);
                    visited.insert(neighbour, count);
                    count
                }
            } + 1)
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

    println!("{}", count(&adj, &mut HashMap::new(), "shiny gold"));
}
