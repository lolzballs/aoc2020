use std::io::{self, BufRead};

type Query<'a> = (char, usize, usize, &'a str);

fn parse_line(line: &str) -> Query {
    use nom::bytes::complete::{is_a, is_not, tag};
    use nom::character::complete::{char, space1};
    use nom::error::ErrorKind;
    use nom::sequence::tuple;

    let (example, (range_lb, _, range_ub, _, character, _)) = tuple::<_, _, (_, ErrorKind), _>((
        is_a("0123456789"),
        char('-'),
        is_a("0123456789"),
        space1,
        is_not(": "),
        tag(": "),
    ))(line)
    .unwrap();

    (
        character.chars().next().unwrap(),
        range_lb.parse::<usize>().unwrap() - 1,
        range_ub.parse::<usize>().unwrap() - 1,
        example,
    )
}

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<_> = {
        let handle = stdin.lock();
        handle.lines().map(|s| s.unwrap()).collect()
    };

    let count: usize = inputs
        .iter()
        .map(|s| s.as_str())
        .map(parse_line)
        .map(|(c, lidx, ridx, example)| {
            example.chars().nth(lidx).map_or(false, |a| a == c)
                ^ example.chars().nth(ridx).map_or(false, |a| a == c)
        })
        .map(|f| if f { 1 } else { 0 })
        .sum();

    println!("{}", count);
}
