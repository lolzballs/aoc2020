use std::convert::From;
use std::io::BufRead;

type State = ((i64, i64), (i64, i64));

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

fn move_ship(times: i64, (x, y): (i64, i64), (waypoint_x, waypoint_y): (i64, i64)) -> (i64, i64) {
    (x + waypoint_x * times, y + waypoint_y * times)
}

fn move_waypoint(inst: Instruction, (x, y): (i64, i64)) -> (i64, i64) {
    match inst.0 {
        Direction::North => (x, y + inst.1),
        Direction::South => (x, y - inst.1),
        Direction::East => (x + inst.1, y),
        Direction::West => (x - inst.1, y),
        Direction::Left | Direction::Right => {
            let turns = {
                let steps = (if inst.0 == Direction::Left {
                    -inst.1
                } else {
                    inst.1
                } / 90)
                    % 4;
                if steps < 0 {
                    4 + steps
                } else {
                    steps
                }
            };

            match turns {
                0 => (x, y),
                1 => (y, -x),
                2 => (-x, -y),
                3 => (-y, x),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn parse_input<I>(input: I) -> impl Iterator<Item = Instruction>
where
    I: Iterator<Item = String>,
{
    input.map(|s| Instruction::from(s.as_str()))
}

fn process_inst((pos, waypoint): State, inst: Instruction) -> State {
    match inst.0 {
        Direction::Forward => (move_ship(inst.1, pos, waypoint), waypoint),
        _ => (pos, move_waypoint(inst, waypoint)),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let final_state =
        parse_input(handle.lines().map(|l| l.unwrap())).fold(((0, 0), (10, 1)), process_inst);
    println!("{:?}", final_state);
}
