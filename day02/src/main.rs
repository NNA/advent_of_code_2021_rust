use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Position {
    hpos: u32,
    depth: u32,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
        Position { hpos: 0, depth: 0 }
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

    let mut position = Position::new();

    for mov in moves {
        match mov.kind {
            MoveKind::Forward => position.hpos += mov.unit,
            MoveKind::Down => position.depth += mov.unit,
            MoveKind::Up => position.depth -= mov.unit,
        }
    }

    println!(
        "At the end : Horizontal position is [{}], Depth is [{}] so the ansmwer is [{}]",
        position.hpos,
        position.depth,
        position.hpos * position.depth
    );
}
