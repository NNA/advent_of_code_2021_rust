use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    #[derive(Debug)]
    enum Status {
        Complete,
        Incomplete,
        Corrupted,
    }

    type Score = u32;

    fn is_closing_for(open_char: char, closing_char: char) -> bool {
        match (open_char, closing_char) {
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
            (_, _) => false,
        }
    }

    fn illegal_char_score(c: char) -> u32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn parse_line(line: &str) -> (Status, Score) {
        let mut stack = vec![];
        let mut status: Status = Status::Incomplete;
        let mut score = 0;

        for c in line.chars() {
            let _res = match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' | ']' | '}' | '>' => match stack.pop() {
                    Some(oc) => match is_closing_for(oc, c) {
                        false => {
                            status = Status::Corrupted;
                            score += illegal_char_score(c);
                            break;
                        }
                        _ => (),
                    },
                    None => (),
                },
                _ => (),
            };
        }
        if stack.len() == 0 {
            status = Status::Complete
        }
        (status, score)
    }

    // Part 1: Find corrupted lines & compute score
    let mut total_score = 0;
    for line in content.lines() {
        let res = parse_line(line);
        total_score += res.1;
    }
    println!("Part 1: solution is : {:?}", total_score);
}
