use std::collections::HashMap;
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

    #[derive(Debug)]
    struct Segment {
        start: Coordinate,
        end: Coordinate,
    }

    impl Segment {
        fn points(&self) -> Vec<Coordinate> {
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
            p
        }
    }

    impl Grid {
        fn new() -> Self {
            Default::default()
        }

        fn add_point(&mut self, coord: Coordinate) {
            // if let Some(row) = self.content.get_mut(row_num as usize) {
            //     if let Some(point) = row.get_mut(col_num as usize) {
            //         *point += 1;
            //     }
            // }
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

    // Fill Grid using segmentList
    for segment in segment_list {
        segment.points().into_iter().for_each(|p| {
            grid.add_point(p);
        });
    }

    // Compute max occurrences
    println!("Part 1: Solution is {:?}", grid.dangerous_points_count());
}
