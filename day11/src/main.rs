use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec;

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

    type EnergyLevel = usize;
    type GridIndex = HashMap<Location, EnergyLevel>;

    #[derive(Debug)]
    struct Grid {
        index: GridIndex,
    }

    #[derive(Debug, PartialEq, Hash, Clone)]
    struct Location {
        column: usize,
        row: usize,
    }

    impl Location {
        fn new(x: usize, y: usize) -> Self {
            Location { column: x, row: y }
        }
    }

    impl Eq for Location {
        // add code here
    }

    impl Grid {
        fn new(content: String) -> Self {
            let mut index = GridIndex::new();
            for (y, line) in content.lines().enumerate() {
                line.chars().enumerate().for_each(|(x, c)| {
                    index.insert(Location::new(x, y), c.to_string().parse().unwrap());
                })
            }
            Grid { index }
        }

        fn do_step(&mut self) -> HashSet<Location> {
            let mut flashing_in_step: HashSet<Location> = HashSet::new();
            let mut increasing_in_step: Vec<Location> = vec![];

            // Increase all octopus energy level by 1
            self.index.iter_mut().for_each(|(_loc, lvl)| {
                *lvl = *lvl + 1;
            });

            // Any octopus with level > 9 Flash & mark neighbors
            self.index.iter().for_each(|(loc, lvl)| {
                if *lvl > 9 {
                    // Flash
                    flashing_in_step.insert(loc.clone());

                    for adj in self.adjacents_of(loc.clone()) {
                        increasing_in_step.push(adj);
                    }
                }
            });

            while let Some(increased_loc) = increasing_in_step.pop() {
                // Increase level by 1
                if let Some(lvl) = self.index.get_mut(&increased_loc) {
                    *lvl = *lvl + 1;
                    if *lvl > 9 {
                        // Try flash and if already flashed we won't process its neighbors
                        if flashing_in_step.insert(increased_loc.clone()) {
                            //Treat Neighbors
                            for adj in self.adjacents_of(increased_loc.clone()) {
                                increasing_in_step.push(adj);
                            }
                        }
                    }
                }
            }

            // Finish
            flashing_in_step.iter().for_each(|loc| {
                if let Some(lvl) = self.index.get_mut(loc) {
                    *lvl = 0;
                }
            });

            flashing_in_step
        }

        fn adjacents_of(&self, loc: Location) -> Vec<Location> {
            let mut v: Vec<Location> = vec![];

            if loc.column > 0 && loc.row > 0 {
                v.push(Location::new(loc.column - 1, loc.row - 1));
            }
            if loc.row > 0 {
                v.push(Location::new(loc.column, loc.row - 1));
            }
            if loc.column < 9 && loc.row > 0 {
                v.push(Location::new(loc.column + 1, loc.row - 1));
            }

            if loc.column > 0 {
                v.push(Location::new(loc.column - 1, loc.row));
            }
            if loc.column < 9 {
                v.push(Location::new(loc.column + 1, loc.row));
            }

            if loc.column > 0 && loc.row < 9 {
                v.push(Location::new(loc.column - 1, loc.row + 1));
            }
            if loc.row < 9 {
                v.push(Location::new(loc.column, loc.row + 1));
            }
            if loc.column < 9 && loc.row < 9 {
                v.push(Location::new(loc.column + 1, loc.row + 1));
            }
            v
        }
    }

    // Part 1: Create map & Search low points
    let mut g = Grid::new(content);
    let mut flash_counter = 0;
    for i in 1..=100 {
        let flashings = g.do_step();
        // println!("Step {} => {:?}", i, flashings.iter().count());
        flash_counter += flashings.iter().count();
    }
    println!("Part 1 : Solution is {:?}", flash_counter);
}
