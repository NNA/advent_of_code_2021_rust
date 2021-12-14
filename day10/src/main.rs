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

    #[derive(Debug, PartialEq)]
    enum Status {
        Complete,
        Incomplete,
        Corrupted,
    }

    type Score = u64;

    fn is_closing_for(open_char: char, closing_char: char) -> bool {
        match (open_char, closing_char) {
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
            (_, _) => false,
        }
    }

    fn matching_closing_char_for(open_char: char) -> char {
        match open_char {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("No Closing caracter matching {}", open_char),
        }
    }

    fn illegal_char_score(c: char) -> u64 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn completed_char_score(c: char) -> u64 {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
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
        if status != Status::Corrupted {
            if stack.len() == 0 {
                status = Status::Complete;
            } else {
                status = Status::Incomplete;
                score = 0;
                while let Some(x) = stack.pop() {
                    let matching_char = matching_closing_char_for(x);
                    score = (score * 5) + completed_char_score(matching_char);
                }
            }
        }
        (status, score)
    }

    // Part 1: Find corrupted lines & compute score
    let mut total_corrupted_score = 0;
    let incomplete_score: u64;
    let mut incomplete_scores: Vec<u64> = vec![];

    for line in content.lines() {
        let res = parse_line(line);
        match res.0 {
            Status::Corrupted => total_corrupted_score += res.1,
            Status::Incomplete => incomplete_scores.push(res.1),
            _ => (),
        }
    }

    incomplete_scores.sort();
    let index = incomplete_scores.len() / 2; // No need to add 1 because vec index starts at 0
    incomplete_score = *incomplete_scores.get(index).unwrap();

    println!("Part 1: solution is : {:?}", total_corrupted_score);
    println!("Part 2: solution is : {:?}", incomplete_score);
}
