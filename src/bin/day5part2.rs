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

    let mut ids: Vec<_> = handle
        .lines()
        .map(|l| l.unwrap())
        .map(|s| decode_seat(&s))
        .map(|(r, c)| r * 8 + c)
        .collect();
    ids.sort();

    let arr = ids
        .windows(3)
        .find(|arr| arr[0] != arr[1] - 1 || arr[2] != arr[1] + 1)
        .unwrap();

    let missing = if arr[0] != arr[1] - 1 {
        arr[1] - 1
    } else {
        arr[1] + 1
    };

    println!("{}", missing);
}
