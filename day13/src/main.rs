use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq)]
enum Direction {
    X,
    Y,
}

type Position = u16;
type Fold = (Direction, Position);
type PaperIndex = Vec<Coordinate>;

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
            let mut iter = coord.split(",");
            index.push(Coordinate {
                x: iter.next().unwrap().to_string().parse().unwrap(),
                y: iter.next().unwrap().to_string().parse().unwrap(),
            });
        }

        for fold in foldings_str.lines() {
            let split = fold.split(" ");
            let mut iter = split.last().unwrap().split("=");
            let direction = match iter.next() {
                Some("x") => Direction::X,
                Some("y") => Direction::Y,
                _ => panic!("invalid input"),
            };
            foldings.push((direction, iter.next().unwrap().parse().unwrap()));
        }

        // So we can pop() all foldings
        foldings.reverse();

        Paper {
            index: index,
            foldings: foldings,
        }
    }

    fn fold_once(&mut self) {
        if let Some((direction, position)) = self.foldings.pop() {
            let mut folded = vec![];
            while let Some(coord) = self.index.pop() {
                match direction {
                    Direction::X => match coord.x.partial_cmp(&position).unwrap() {
                        Ordering::Greater => {
                            folded.push(Coordinate {
                                x: position - (coord.x - position),
                                y: coord.y,
                            });
                        }
                        Ordering::Less => {
                            folded.push(Coordinate {
                                x: coord.x,
                                y: coord.y,
                            });
                        }
                        Ordering::Equal => (),
                    },
                    Direction::Y => match coord.y.partial_cmp(&position).unwrap() {
                        Ordering::Greater => {
                            folded.push(Coordinate {
                                x: coord.x,
                                y: position - (coord.y - position),
                            });
                        }
                        Ordering::Less => {
                            folded.push(Coordinate {
                                x: coord.x,
                                y: coord.y,
                            });
                        }
                        Ordering::Equal => (),
                    },
                };
            }
            folded.sort();
            folded.dedup();
            self.index = folded;
        }
    }

    fn visible_dot_count(self) -> usize {
        self.index.len()
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

    let mut paper: Paper = Paper::new(content);
    paper.fold_once();
    let result = paper.visible_dot_count();
    println!("Part1 result is {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::{Coordinate, Direction, Paper};
    // use std::collections::HashSet;

    const SIMPLE_CONTENT: &str = r#"6,10
0,14

fold along y=7"#;

    #[test]
    fn it_can_properly_initialize_a_paper() {
        let expected_paper: Paper = Paper::new(SIMPLE_CONTENT.to_string());

        let mut i = vec![];
        i.push(Coordinate { x: 6, y: 10 });
        i.push(Coordinate { x: 0, y: 14 });

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

    #[test]
    fn it_can_fold() {
        let mut expected_paper: Paper = Paper::new(SIMPLE_CONTENT.to_string());
        expected_paper.fold();

        let mut i = vec![];
        i.push(Coordinate { x: 0, y: 0 });
        i.push(Coordinate { x: 6, y: 4 });

        let f = vec![];

        assert_eq!(
            expected_paper,
            Paper {
                index: i,
                foldings: f
            }
        );
    }
}
