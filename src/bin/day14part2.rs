use std::collections::BTreeMap;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Set(usize, u64),
}

struct Memory {
    mem: BTreeMap<usize, u64>,
    bitmask: Option<(u64, u64, Vec<u64>)>,
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
        let mut floating_bits = 0;
        let mut floating = Vec::new();
        for (bit, val) in bitstring.chars().rev().enumerate() {
            match val {
                '1' => ones = ones | (1 << bit),
                '0' => (),
                'X' => {
                    floating_bits = floating_bits | (1 << bit);
                    floating.push(bit as u64);
                }
                _ => panic!("invalid character in bitmask"),
            }
        }

        self.bitmask = Some((ones, floating_bits, floating));
    }

    fn set(&mut self, entry: usize, value: u64) {
        if self.bitmask.is_none() {
            self.mem.insert(entry, value);
            return;
        }

        let bitmask = self.bitmask.as_ref().unwrap();

        let entry = ((entry as u64) | bitmask.0) & !bitmask.1;
        let mut counter = 0;
        let entries_iter = std::iter::from_fn(move || {
            if counter >= (1 << bitmask.2.len()) {
                return None;
            }

            let val = bitmask
                .2
                .iter()
                .enumerate()
                .map(|(i, bit)| ((counter & (1 << i)) >> i) << bit)
                .fold(entry, |val, set_mask| val | set_mask);

            counter += 1;

            Some(val as usize)
        });

        for entry in entries_iter {
            self.mem.insert(entry, value);
        }
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

    let sum: u64 = mem.mem.values().map(|v| v & 0xFFFFFFFFF).sum();
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
        mem.set(1, 5);
        mem.mask("000000000000000000000000000000X1001X");
        assert_eq!(mem.mem.get(&1), Some(&5));

        // 0b1 => 0b010010, 0b010011, 0b110010, 0b110011
        mem.set(1, 10);
        assert_eq!(mem.mem.get(&0b010010), Some(&10));
        assert_eq!(mem.mem.get(&0b010011), Some(&10));
        assert_eq!(mem.mem.get(&0b110010), Some(&10));
        assert_eq!(mem.mem.get(&0b110011), Some(&10));
    }
}
