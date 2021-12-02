use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Position {
    aim: u32,
    depth: u32,
    hpos: u32,
}

#[derive(Debug, Clone)]
pub struct Move {
    kind: MoveKind,
    unit: u32,
}

impl Position {
    fn new() -> Self {
        Default::default()
    }
}

impl Move {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone)]
enum MoveKind {
    Forward,
    Down,
    Up,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let mut iter = input.split(' ');
        let mut mov = Move::new();

        mov.kind = match iter.next() {
            Some(maybe_move) => match maybe_move {
                "forward" => MoveKind::Forward,
                "down" => MoveKind::Down,
                "up" => MoveKind::Up,
                _ => {
                    panic!("invalid move");
                }
            },
            None => panic!("invalid input"),
        };

        mov.unit = match iter.next() {
            Some(maybe_unit) => match maybe_unit.parse() {
                Ok(unit) => unit,
                Err(_e) => panic!("invalid unit"),
            },
            None => panic!("invalid input"),
        };

        mov
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            aim: 0,
            hpos: 0,
            depth: 0,
        }
    }
}

impl Default for Move {
    fn default() -> Self {
        Move {
            kind: MoveKind::Forward,
            unit: 0,
        }
    }
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let moves: Vec<Move> = content.split('\n').map(|x| Move::from(x)).collect();
    let moves_part2 = moves.clone();

    let mut position_part1 = Position::new();
    let mut position_part2 = Position::new();

    // Part 1
    for mov in moves {
        match mov.kind {
            MoveKind::Forward => position_part1.hpos += mov.unit,
            MoveKind::Down => position_part1.depth += mov.unit,
            MoveKind::Up => position_part1.depth -= mov.unit,
        }
    }

    println!(
        "PART 1 - At the end : Horizontal position is [{}], Depth is [{}] so the ansmwer is [{}]",
        position_part1.hpos,
        position_part1.depth,
        position_part1.hpos * position_part1.depth
    );

    // Part 2
    for mov in moves_part2 {
        match mov.kind {
            MoveKind::Forward => {
                position_part2.hpos += mov.unit;
                position_part2.depth += position_part2.aim * mov.unit;
            }
            MoveKind::Down => position_part2.aim += mov.unit,
            MoveKind::Up => position_part2.aim -= mov.unit,
        }
    }

    println!(
        "PART 2 - At the end : Horizontal position is [{}], Depth is [{}] so the ansmwer is [{}]",
        position_part2.hpos,
        position_part2.depth,
        position_part2.hpos * position_part2.depth
    );
}
