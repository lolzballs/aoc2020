use std::convert::From;
use std::io::BufRead;

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

type State = (Direction, i64, i64);

struct Instruction(Direction, i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let dir = match s.as_bytes()[0] {
            b'N' => Direction::North,
            b'S' => Direction::South,
            b'E' => Direction::East,
            b'W' => Direction::West,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            b'F' => Direction::Forward,
            _ => panic!(),
        };

        Self(dir, s[1..].parse().unwrap())
    }
}

fn move_direction(inst: Instruction, x: i64, y: i64) -> (i64, i64) {
    match inst.0 {
        Direction::North => (x, y + inst.1),
        Direction::South => (x, y - inst.1),
        Direction::East => (x + inst.1, y),
        Direction::West => (x - inst.1, y),
        _ => panic!(),
    }
}

fn rotate(inst: Instruction, dir: Direction) -> Direction {
    let idx = match inst.0 {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => panic!(),
    } * (inst.1 / 90)
        % 4;

    let start = match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
        _ => panic!(),
    };

    let full = (start + idx) % COMPASS.len() as i64;
    if full < 0 {
        COMPASS[(COMPASS.len() as i64 + full) as usize]
    } else {
        COMPASS[full as usize]
    }
}

fn parse_input<I>(input: I) -> impl Iterator<Item = Instruction>
where
    I: Iterator<Item = String>,
{
    input.map(|s| Instruction::from(s.as_str()))
}

fn process_inst((dir, x, y): State, inst: Instruction) -> State {
    let (dir, (x, y)) = match inst.0 {
        Direction::Left | Direction::Right => (rotate(inst, dir), (x, y)),
        Direction::Forward => (dir, move_direction(Instruction(dir, inst.1), x, y)),
        _ => (dir, move_direction(inst, x, y)),
    };
    (dir, x, y)
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let final_state =
        parse_input(handle.lines().map(|l| l.unwrap())).fold((Direction::East, 0, 0), process_inst);
    println!("{:?}", final_state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(
            rotate(Instruction(Direction::Left, 90), Direction::North),
            Direction::West
        );
        assert_eq!(
            rotate(Instruction(Direction::Right, 90), Direction::North),
            Direction::East
        );
        assert_eq!(
            rotate(Instruction(Direction::Left, 90), Direction::South),
            Direction::East
        );
        assert_eq!(
            rotate(Instruction(Direction::Right, 90), Direction::South),
            Direction::West
        );
        assert_eq!(
            rotate(Instruction(Direction::Right, 360), Direction::East),
            Direction::East
        );
        assert_eq!(
            rotate(Instruction(Direction::Left, 360), Direction::East),
            Direction::East
        );
    }
}
