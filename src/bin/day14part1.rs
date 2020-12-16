use std::collections::BTreeMap;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Set(usize, u64),
}

struct Memory {
    mem: BTreeMap<usize, u64>,
    bitmask: Option<(u64, u64)>,
}

impl Memory {
    fn new() -> Self {
        Memory {
            mem: BTreeMap::new(),
            bitmask: None,
        }
    }

    fn mask(&mut self, bitstring: &str) {
        let mut ones = 0;
        let mut zeros = 0;
        for (bit, val) in bitstring.chars().rev().enumerate() {
            match val {
                '1' => ones = ones | (1 << bit),
                '0' => zeros = zeros | (1 << bit),
                'X' => (),
                _ => panic!("invalid character in bitmask"),
            }
        }

        self.bitmask = Some((ones, zeros));
    }

    fn set(&mut self, entry: usize, value: u64) {
        let value = match self.bitmask {
            Some((ones, zeros)) => (value | ones) & !zeros,
            None => value,
        };

        self.mem.insert(entry, value);
    }
}

fn parse_line(line: &str) -> Instruction {
    let mut split = line.split(" = ");

    match split.next().unwrap() {
        "mask" => Instruction::Mask(split.next().unwrap()),
        s => {
            if &s[..3] != "mem" {
                panic!();
            }
            Instruction::Set(
                s[4..s.len() - 1].parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let mut mem = Memory::new();
    for line in handle.lines() {
        let line = line.unwrap();
        let inst = parse_line(&line);
        match inst {
            Instruction::Mask(mask) => mem.mask(mask),
            Instruction::Set(loc, val) => mem.set(loc, val),
        }
    }

    let sum: u64 = mem.mem.values().sum();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_no_mask() {
        let mut mem = Memory::new();
        mem.set(1, 5);
        assert_eq!(mem.mem.get(&1), Some(&5));
        mem.set(1, 10);
        assert_eq!(mem.mem.get(&1), Some(&10));
        mem.set(2, 20);
        assert_eq!(mem.mem.get(&1), Some(&10));
        assert_eq!(mem.mem.get(&2), Some(&20));
    }

    #[test]
    fn set_mask() {
        let mut mem = Memory::new();
        mem.set(1, 5); // 0b101
        mem.mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mem.mem.get(&1), Some(&5));
        mem.set(1, 10); // 0b1010
        assert_eq!(mem.mem.get(&1), Some(&0b1001000));
        mem.set(2, 20); // 0b10100
        assert_eq!(mem.mem.get(&1), Some(&0b1001000));
        assert_eq!(mem.mem.get(&2), Some(&0b1010100));
    }
}
