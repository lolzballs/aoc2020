use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[..4] {
            "acc " => Instruction::Acc(s[4..].parse().map_err(|_| "bad operand".to_owned())?),
            "jmp " => Instruction::Jmp(s[4..].parse().map_err(|_| "bad operand".to_owned())?),
            "nop " => Instruction::Nop,
            _ => return Err("bad opcode".to_owned()),
        })
    }
}

fn run(instructions: Vec<Instruction>) -> i64 {
    let mut acc = 0;
    let mut pc: isize = 0;

    let mut visited = HashSet::new();

    while visited.insert(pc) {
        match instructions[pc as usize] {
            Instruction::Acc(val) => acc += val,
            Instruction::Jmp(val) => pc += val - 1,
            Instruction::Nop => (),
        }
        pc += 1;
    }

    acc
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let instructions: Vec<Instruction> = handle
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    println!("{}", run(instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        assert_eq!("acc +1".parse(), Ok(Instruction::Acc(1)));
        assert_eq!("acc -1".parse(), Ok(Instruction::Acc(-1)));
        assert_eq!("jmp +2".parse(), Ok(Instruction::Jmp(2)));
        assert_eq!("jmp -2".parse(), Ok(Instruction::Jmp(-2)));
        assert_eq!("nop +2".parse(), Ok(Instruction::Nop));
        assert_eq!("nop -2".parse(), Ok(Instruction::Nop));
        assert_eq!(
            "acc a".parse::<Instruction>(),
            Err("bad operand".to_owned())
        );
        assert_eq!(
            "asdfa aasdf".parse::<Instruction>(),
            Err("bad opcode".to_owned())
        );
    }

    #[test]
    fn test_run() {
        assert_eq!(
            run(vec![
                Instruction::Nop,
                Instruction::Nop,
                Instruction::Jmp(-2)
            ]),
            0
        );
        assert_eq!(
            run(vec![
                Instruction::Nop,
                Instruction::Acc(1),
                Instruction::Jmp(4),
                Instruction::Acc(3),
                Instruction::Jmp(-3),
                Instruction::Acc(-99),
                Instruction::Acc(1),
                Instruction::Jmp(-4),
                Instruction::Acc(6),
            ]),
            5
        );
    }
}
