use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, Read};
use std::path::Path;

#[derive(Debug, PartialEq, Hash, Clone)]
struct Coordinate {
    x: i16,
    y: i16,
}

#[derive(Debug, PartialEq)]
enum Direction {
    X,
    Y,
}

type Position = u16;
type Fold = (Direction, Position);
type PaperIndex = HashSet<Coordinate>;

#[derive(Debug, PartialEq)]
struct Paper {
    index: PaperIndex,
    foldings: Vec<Fold>,
}

impl Paper {
    fn new(content: String) -> Self {
        let mut index = PaperIndex::new();
        let mut foldings = vec![];

        let mut split = content.split("\n\n");
        let coords_str = split.next().unwrap();
        let foldings_str = split.next().unwrap();

        for coord in coords_str.lines() {
            println!("coord {:?}", coord);
            // if line == "" {
            //     break;
            // }
            let mut iter = coord.split(",");
            index.insert(Coordinate {
                x: iter.next().unwrap().to_string().parse().unwrap(),
                y: iter.next().unwrap().to_string().parse().unwrap(),
            });
        }

        for fold in foldings_str.lines() {
            println!("fold {:?}", fold);
            let split = fold.split(" ");
            let mut iter = split.last().unwrap().split("=");
            let direction = match iter.next() {
                Some("x") => Direction::X,
                Some("y") => Direction::Y,
                _ => panic!("invalid input"),
            };
            foldings.push((direction, iter.next().unwrap().parse().unwrap()));
        }

        Paper {
            index: index,
            foldings: foldings,
        }
    }

    fn fold(&self) {
        println!("doing nothing");
    }
}

impl Eq for Coordinate {
    // add code here
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("input_test.txt");
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

    let paper: Paper = Paper::new(content);
}

#[cfg(test)]
mod tests {
    use crate::{Coordinate, Direction, Paper};
    use std::collections::HashSet;

    const CONTENT: &str = r#"6,10
0,14

fold along y=7"#;

    #[test]
    fn it_can_properly_initialize_a_paper() {
        let expected_paper: Paper = Paper::new(CONTENT.to_string());

        let mut i = HashSet::new();
        i.insert(Coordinate { x: 6, y: 10 });
        i.insert(Coordinate { x: 0, y: 14 });

        let mut f = vec![];
        f.push((Direction::Y, 7));

        assert_eq!(
            expected_paper,
            Paper {
                index: i,
                foldings: f
            }
        );
    }

    // #[test]
    // fn it_can_fold_once() {
    //     let paper: Paper = Paper::new(CONTENT.to_string());
    //     paper.fold();

    //     assert_eq!(2 + 2, 4);
    // }
}
