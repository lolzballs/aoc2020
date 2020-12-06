use std::io::BufRead;

fn decode_seat(bs: &str) -> (usize, usize) {
    let rows = &bs[..7];
    let cols = &bs[7..];

    let row_range = rows.chars().fold(0..128, |r, c| {
        let mid = (r.end + r.start) / 2;
        match c {
            'F' => r.start..mid,
            'B' => mid..r.end,
            _ => unreachable!(),
        }
    });
    let col_range = cols.chars().fold(0..8, |r, c| {
        let mid = (r.end + r.start) / 2;
        match c {
            'L' => r.start..mid,
            'R' => mid..r.end,
            _ => unreachable!(),
        }
    });

    (row_range.start, col_range.start)
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let max = handle
        .lines()
        .map(|l| l.unwrap())
        .map(|s| decode_seat(&s))
        .map(|(r, c)| r * 8 + c)
        .max();

    println!("{}", max.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode1() {
        assert_eq!(decode_seat("BFFFBBFRRR"), (70, 7));
    }

    #[test]
    fn decode2() {
        assert_eq!(decode_seat("FFFBBBFRRR"), (14, 7));
    }

    #[test]
    fn decode3() {
        assert_eq!(decode_seat("BBFFBBFRLL"), (102, 4));
    }
}
