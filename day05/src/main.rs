#![feature(int_abs_diff)]

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    type GridIndex = HashMap<Coordinate, u16>;

    #[derive(Debug, Clone)]
    struct Grid {
        index: GridIndex,
        content: Vec<Vec<u16>>,
    }

    #[derive(Debug, PartialEq, Hash, Clone)]
    struct Coordinate {
        row: u16,
        column: u16,
    }

    impl Eq for Coordinate {
        // add code here
    }

    #[derive(Debug, Clone)]
    struct Segment {
        start: Coordinate,
        end: Coordinate,
    }
    //20687-7142=13545

    impl Segment {
        fn points(&self, counting_diagonals: bool) -> Vec<Coordinate> {
            let mut p = vec![];
            if self.start.row == self.end.row {
                let range;
                if self.start.column <= self.end.column {
                    range = self.start.column..=self.end.column;
                } else {
                    range = self.end.column..=self.start.column;
                }
                // Rows are same => lets compute points by using columns between
                for c in range {
                    p.push(Coordinate {
                        row: self.start.row,
                        column: c,
                    });
                }
            }
            if self.start.column == self.end.column {
                let range;
                if self.start.row <= self.end.row {
                    range = self.start.row..=self.end.row;
                } else {
                    range = self.end.row..=self.start.row;
                }
                // Columns are same => lets compute points by using rows between
                for r in range {
                    p.push(Coordinate {
                        row: r,
                        column: self.start.column,
                    });
                }
            }

            if counting_diagonals && self.is_diagonal_45() {
                let row_range;
                let col_range;
                // We are on a 45 diagnonal

                // Determine ranges
                if self.start.row <= self.end.row {
                    row_range = self.start.row..=self.end.row;
                } else {
                    row_range = self.end.row..=self.start.row;
                }
                if self.start.column <= self.end.column {
                    col_range = self.start.column..=self.end.column;
                } else {
                    col_range = self.end.column..=self.start.column;
                }

                if (self.start.row + self.start.column != self.end.row + self.end.column) {
                    let min_col_range = col_range.min().unwrap();
                    for (i, r) in row_range.enumerate() {
                        p.push(Coordinate {
                            row: r,
                            column: min_col_range + i as u16,
                        });
                    }
                } else {
                    // self.start.row + self.start.column == self.end.row + self.end.column
                    let max_col_range = col_range.max().unwrap();

                    for (i, r) in row_range.enumerate() {
                        p.push(Coordinate {
                            row: r,
                            column: max_col_range - i as u16,
                        });
                    }
                }
            }

            p
        }

        fn is_diagonal_45(&self) -> bool {
            self.end.row.abs_diff(self.start.row) == self.end.column.abs_diff(self.start.column)
        }
    }

    impl Grid {
        fn new() -> Self {
            Default::default()
        }

        fn add_point(&mut self, coord: Coordinate) {
            if let Some(value) = self.index.get_mut(&coord) {
                *value += 1;
            } else {
                self.index.insert(coord, 1);
            }
        }

        fn dangerous_points_count(&mut self) -> u16 {
            self.index.values().filter(|x| **x >= 2u16).count() as u16
        }
    }

    impl Default for Grid {
        fn default() -> Self {
            Grid {
                index: HashMap::new(),
                content: vec![vec![]],
            }
        }
    }

    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut grid = String::new();
            for i in 0..=9 {
                for j in 0..=9 {
                    // println!("displaying row {:?} column {:?}", i, j);
                    match self.index.get(&Coordinate { row: j, column: i }) {
                        Some(v) => grid.push_str(&v.to_string()),
                        None => grid.push_str("."),
                    }
                }
                grid.push_str("\n");
            }
            write!(f, "{}", grid)
        }
    }

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

    let mut segment_list: Vec<Segment> = vec![];

    // Parse input & prepare SegmentList
    for (input_line) in content.split('\n') {
        let mut iter = input_line.split(" -> ");

        // Parse before "->" : the source
        let source = iter.next().unwrap();
        let mut source_iter = source.split(",");
        let source_x = source_iter.next().unwrap().parse().unwrap();
        let source_y = source_iter.next().unwrap().parse().unwrap();
        let source_coord = Coordinate {
            row: source_x,
            column: source_y,
        };

        // Parse after "->" : the destination
        let dest = iter.next().unwrap();
        let mut dest_iter = dest.split(",");
        let dest_x = dest_iter.next().unwrap().parse().unwrap();
        let dest_y = dest_iter.next().unwrap().parse().unwrap();
        let dest_coord = Coordinate {
            row: dest_x,
            column: dest_y,
        };

        segment_list.push(Segment {
            start: source_coord,
            end: dest_coord,
        });
    }

    let mut grid = Grid::new();

    let mut part2_segment_list = segment_list.clone();

    // Part 1 : Fill Grid using segmentList and NOT counting diagonal lines
    for segment in segment_list {
        segment.points(false).into_iter().for_each(|p| {
            grid.add_point(p);
        });
    }

    // Compute max occurrences
    println!("Part 1: Solution is {:?}", grid.dangerous_points_count());

    // Part 2 : Fill Grid using segmentList and counting diagonal lines
    let mut grid_part2 = Grid::new();

    for segment in part2_segment_list {
        segment.points(true).into_iter().for_each(|p| {
            grid_part2.add_point(p);
        });
    }

    println!(
        "Part 2: Solution is {:?}",
        grid_part2.dangerous_points_count()
    );
}
