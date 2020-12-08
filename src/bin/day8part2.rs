use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn swap(&mut self) {
        let repl = match self {
            Instruction::Jmp(i) => Instruction::Nop(*i),
            Instruction::Nop(i) => Instruction::Jmp(*i),
            Instruction::Acc(i) => Instruction::Acc(*i),
        };

        *self = repl;
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[..4] {
            "acc " => Instruction::Acc(s[4..].parse().map_err(|_| "bad operand".to_owned())?),
            "jmp " => Instruction::Jmp(s[4..].parse().map_err(|_| "bad operand".to_owned())?),
            "nop " => Instruction::Nop(s[4..].parse().map_err(|_| "bad operand".to_owned())?),
            _ => return Err("bad opcode".to_owned()),
        })
    }
}

fn run(instructions: &Vec<Instruction>) -> (bool, i64) {
    let mut acc = 0;
    let mut pc: isize = 0;

    let mut visited = HashSet::new();

    while visited.insert(pc) {
        match instructions[pc as usize] {
            Instruction::Acc(val) => acc += val,
            Instruction::Jmp(val) => pc += val - 1,
            Instruction::Nop(_) => (),
        }
        pc += 1;

        if pc as usize == instructions.len() {
            return (true, acc);
        }
    }

    (false, acc)
}

fn find_swaps(instructions: &Vec<Instruction>) -> impl Iterator<Item = usize> + '_ {
    instructions
        .iter()
        .enumerate()
        .filter_map(|(i, inst)| match inst {
            Instruction::Nop(_) | Instruction::Jmp(_) => Some(i),
            _ => None,
        })
}

fn attempt_swaps(mut instructions: Vec<Instruction>) -> Option<i64> {
    for idx in find_swaps(&instructions).collect::<Vec<_>>() {
        instructions[idx].swap();
        let (halted, acc) = run(&instructions);
        if halted {
            return Some(acc);
        }
        instructions[idx].swap();
    }
    None
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let instructions: Vec<Instruction> = handle
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    println!("{:?}", attempt_swaps(instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_swap() {
        {
            let mut acc = Instruction::Acc(1);
            acc.swap();
            assert_eq!(acc, Instruction::Acc(1));
        }
        {
            let mut nop = Instruction::Nop(1);
            nop.swap();
            assert_eq!(nop, Instruction::Jmp(1));
        }
        {
            let mut jmp = Instruction::Jmp(1);
            jmp.swap();
            assert_eq!(jmp, Instruction::Nop(1));
        }
    }

    #[test]
    fn test_decoding() {
        assert_eq!("acc +1".parse(), Ok(Instruction::Acc(1)));
        assert_eq!("acc -1".parse(), Ok(Instruction::Acc(-1)));
        assert_eq!("jmp +2".parse(), Ok(Instruction::Jmp(2)));
        assert_eq!("jmp -2".parse(), Ok(Instruction::Jmp(-2)));
        assert_eq!("nop +2".parse(), Ok(Instruction::Nop(2)));
        assert_eq!("nop -2".parse(), Ok(Instruction::Nop(-2)));
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
            run(&vec![
                Instruction::Nop(1),
                Instruction::Nop(1),
                Instruction::Jmp(-2)
            ]),
            (false, 0)
        );
        assert_eq!(
            run(&vec![
                Instruction::Nop(1),
                Instruction::Acc(1),
                Instruction::Jmp(4),
                Instruction::Acc(3),
                Instruction::Jmp(-3),
                Instruction::Acc(-99),
                Instruction::Acc(1),
                Instruction::Jmp(-4),
                Instruction::Acc(6),
            ]),
            (false, 5)
        );
        assert_eq!(
            run(&vec![
                Instruction::Nop(1),
                Instruction::Acc(1),
                Instruction::Jmp(4),
                Instruction::Acc(3),
                Instruction::Jmp(-3),
                Instruction::Acc(-99),
                Instruction::Acc(1),
                Instruction::Nop(1),
                Instruction::Acc(6),
            ]),
            (true, 8)
        );
    }

    #[test]
    fn test_find_swaps() {
        assert_eq!(
            find_swaps(&vec![
                Instruction::Nop(1),
                Instruction::Acc(1),
                Instruction::Jmp(4),
                Instruction::Acc(3),
                Instruction::Jmp(-3),
                Instruction::Acc(-99),
                Instruction::Acc(1),
                Instruction::Nop(1),
                Instruction::Acc(6),
            ])
            .collect::<Vec<_>>(),
            vec![0, 2, 4, 7]
        );
    }

    #[test]
    fn test_attempt_swaps() {
        assert_eq!(
            attempt_swaps(&mut vec![
                Instruction::Nop(1),
                Instruction::Acc(1),
                Instruction::Jmp(4),
                Instruction::Acc(3),
                Instruction::Jmp(-3),
                Instruction::Acc(-99),
                Instruction::Acc(1),
                Instruction::Jmp(-4),
                Instruction::Acc(6),
            ]),
            Some(8)
        );
    }
}
