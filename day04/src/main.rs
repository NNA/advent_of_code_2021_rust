use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    type GridIndex = HashMap<u16, Coordinate>;

    #[derive(Debug, Clone)]
    struct Grid {
        row_count: Option<u16>,
        column_count: Option<u16>,
        index: GridIndex,
        content: [[Option<u16>; 5]; 5],
    }

    #[derive(Debug, PartialEq, Hash, Clone)]
    struct Coordinate {
        row: u16,
        column: u16,
    }

    impl Eq for Coordinate {}

    impl Grid {
        fn new() -> Self {
            Default::default()
        }

        fn add_number(&mut self, row: u16, column: u16, value: u16) {
            let coord = Coordinate {
                row: row,
                column: column,
            };
            self.index.insert(value, coord);
            self.content[row as usize][column as usize] = Some(value);
        }

        fn remove_number(&mut self, value: u16) -> Option<Coordinate> {
            // Seek & Remove from index
            match self.index.remove(&value) {
                None => None,
                Some(coord) => {
                    // Remove from content
                    self.content[coord.row as usize][coord.column as usize] = None;
                    Some(coord)
                }
            }
        }

        fn is_row_complete(&mut self, row_number: u16) -> bool {
            self.content[row_number as usize] == [None, None, None, None, None]
        }

        fn is_column_complete(&mut self, col_number: u16) -> bool {
            self.content[0][col_number as usize] == None
                && self.content[1][col_number as usize] == None
                && self.content[2][col_number as usize] == None
                && self.content[3][col_number as usize] == None
                && self.content[4][col_number as usize] == None
        }
    }

    impl Default for Grid {
        fn default() -> Self {
            Grid {
                row_count: None,
                column_count: None,
                index: HashMap::new(),
                content: [[Some(0 as u16); 5]; 5],
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

    let mut numbers: Vec<u16> = vec![];
    let mut grids: Vec<Grid> = vec![];
    let mut row_number = 0;

    // Part 1
    // Parse input & prepare grids
    for (pos, input_part) in content.split('\n').enumerate() {
        if pos == 0 {
            // First the numbers
            numbers = input_part
                .split(',')
                .map(|x| -> u16 { x.parse().unwrap() })
                .collect();
        } else {
            if input_part == "" {
                match grids.pop() {
                    Some(grid) => {
                        grids.push(grid);
                        row_number = 0;
                    }
                    None => (),
                }
                grids.push(Grid::new());
            } else {
                // Input is not empty
                // We are on a grid row
                match grids.pop() {
                    Some(mut grid) => {
                        for (col, number_str) in input_part.split_whitespace().enumerate() {
                            grid.add_number(row_number, col as u16, number_str.parse().unwrap())
                        }
                        grids.push(grid);
                        row_number += 1;
                    }
                    None => (),
                }
            }
        }
    }

    // Check grids
    'outer: for number in numbers {
        for grid in &mut grids {
            let coord = &grid.remove_number(number);
            if coord.is_some() {
                let c = coord.as_ref().unwrap();
                if grid.is_row_complete(c.row) || grid.is_column_complete(c.column) {
                    let sum: u16 = grid.index.keys().sum();
                    println!(
                        "For Part 1 : sum is [{}], last number was [{}] so solution is [{}]",
                        sum,
                        number,
                        sum * number
                    );
                    break 'outer;
                }
            }
        }
    }
}
