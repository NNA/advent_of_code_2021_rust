use core::fmt;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    X,
    Y,
}

type Position = u16;
type Fold = (Direction, Position);
type PaperIndex = HashSet<Coordinate>;

#[derive(Debug, PartialEq, Clone)]
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
            let mut iter = coord.split(',');
            let x = iter.next().unwrap().to_string().parse().unwrap();
            let y = iter.next().unwrap().to_string().parse().unwrap();
            index.insert(Coordinate { x, y });
        }

        for fold in foldings_str.lines() {
            let split = fold.split(' ');
            let mut iter = split.last().unwrap().split('=');
            let direction = match iter.next() {
                Some("x") => Direction::X,
                Some("y") => Direction::Y,
                _ => panic!("invalid input"),
            };
            foldings.push((direction, iter.next().unwrap().parse().unwrap()));
        }

        Paper { index, foldings }
    }

    fn fold_once_at(&mut self, direction: Direction, position: Position) {
        let mut folded = HashSet::new();
        self.index.iter().for_each(|coord| {
            match direction {
                Direction::X => match coord.x.partial_cmp(&position).unwrap() {
                    Ordering::Greater => {
                        folded.insert(Coordinate {
                            x: position - (coord.x - position),
                            y: coord.y,
                        });
                    }
                    Ordering::Less => {
                        folded.insert(Coordinate {
                            x: coord.x,
                            y: coord.y,
                        });
                    }
                    Ordering::Equal => (),
                },
                Direction::Y => match coord.y.partial_cmp(&position).unwrap() {
                    Ordering::Greater => {
                        folded.insert(Coordinate {
                            x: coord.x,
                            y: position - (coord.y - position),
                        });
                    }
                    Ordering::Less => {
                        folded.insert(Coordinate {
                            x: coord.x,
                            y: coord.y,
                        });
                    }
                    Ordering::Equal => (),
                },
            };
        });
        self.index = folded;
    }

    fn fold(&mut self, limit: Option<u16>) {
        for (count, (direction, position)) in self.foldings.clone().into_iter().enumerate() {
            self.fold_once_at(direction, position);
            if limit.is_some() && count == (limit.unwrap() - 1).into() {
                break;
            }
        }
    }

    fn visible_dot_count(self) -> usize {
        self.index.len()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = String::new();
        for i in 0..=6 {
            for j in 0..=40 {
                // println!("displaying row {:?} column {:?}", i, j);
                match self.index.get(&Coordinate { x: j, y: i }) {
                    Some(_v) => grid.push('#'),
                    None => grid.push('.'),
                }
            }
            grid.push('\n');
        }
        write!(f, "{}", grid)
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
    if let Err(why) = file.read_to_string(&mut content) {
        panic!("couldn't read {}: {}", display, why)
    }

    // Part 1
    let mut paper: Paper = Paper::new(content);
    let mut paper_part2 = paper.clone();
    paper.fold(Some(1));
    let sol1 = paper.visible_dot_count();
    println!("Part1 solution is {:?}", sol1);

    // Part 2
    paper_part2.fold(None);
    println!("Part2 solution is \n{:}", paper_part2);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{Coordinate, Direction, Paper};
    // use std::collections::HashSet;

    const SIMPLE_CONTENT: &str = r#"6,10
0,14

fold along y=7"#;

    #[test]
    fn it_can_properly_initialize_a_paper() {
        let expected_paper: Paper = Paper::new(SIMPLE_CONTENT.to_string());

        let mut index = HashSet::new();
        index.insert(Coordinate { x: 6, y: 10 });
        index.insert(Coordinate { x: 0, y: 14 });

        let mut foldings = vec![];
        foldings.push((Direction::Y, 7));

        assert_eq!(expected_paper, Paper { index, foldings });
    }

    #[test]
    fn it_can_fold() {
        let mut expected_paper: Paper = Paper::new(SIMPLE_CONTENT.to_string());
        expected_paper.fold(Some(1));

        let mut index = HashSet::new();
        index.insert(Coordinate { x: 0, y: 0 });
        index.insert(Coordinate { x: 6, y: 4 });

        let mut foldings = vec![];
        foldings.push((Direction::Y, 7));

        assert_eq!(expected_paper, Paper { index, foldings });
    }
}
