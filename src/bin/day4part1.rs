use std::io::Read;

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();

    let mandatory = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    let count: usize = input
        .split("\n\n")
        .map(|p| p.replace('\n', " "))
        .map(|p| mandatory.iter().map(|f| p.contains(f)).any(|v| !v))
        .map(|invalid| if !invalid { 1 } else { 0 })
        .sum();

    println!("{}", count);
}
