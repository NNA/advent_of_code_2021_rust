#![feature(int_abs_diff)]

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
    struct CrabsArmy {
        positions: Vec<usize>,
    }

    impl CrabsArmy {
        fn new(positions_str: String) -> Self {
            let start_positions: Vec<usize> = positions_str
                .split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            CrabsArmy {
                positions: start_positions,
            }
        }

        fn min_position(&self) -> usize {
            *self.positions.iter().min().unwrap()
        }

        fn max_position(&self) -> usize {
            *self.positions.iter().max().unwrap()
        }

        fn least_fuel_to_align(&self) -> usize {
            let mut army_lowest_required_fuel = usize::MAX;
            for candidate_pos in self.min_position()..=self.max_position() {
                let army_candidate_required_fuel = self
                    .positions
                    .iter()
                    .fold(0usize, |sum, current| sum + current.abs_diff(candidate_pos));

                if army_candidate_required_fuel < army_lowest_required_fuel {
                    army_lowest_required_fuel = army_candidate_required_fuel;
                }
            }
            army_lowest_required_fuel
        }
    }

    // // Part 1
    let army = CrabsArmy::new(content);
    println!("Least fuel {}", army.least_fuel_to_align());
}
