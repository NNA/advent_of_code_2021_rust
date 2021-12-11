use std::collections::HashMap;
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

    type Height = usize;
    type RiskLevel = usize;
    type MapIndex = HashMap<Location, Height>;
    type LowestMap = HashMap<Location, RiskLevel>;

    #[derive(Debug)]
    struct Map {
        row_count: usize,
        col_count: usize,
        index: MapIndex,
        lowest: Option<LowestMap>,
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

    impl Map {
        fn new(content: String) -> Self {
            let mut index = MapIndex::new();
            for (y, line) in content.split('\n').enumerate() {
                line.chars().enumerate().for_each(|(x, c)| {
                    let loc = Location::new(x, y);
                    index.insert(loc, c.to_string().parse().unwrap());
                })
            }
            Map {
                row_count: content.matches('\n').count() + 1,
                col_count: content.find('\n').unwrap(),
                index,
                lowest: None,
            }
        }

        fn compute_lowest_points(&mut self) {
            let mut lm = LowestMap::new();

            self.index.iter().for_each(|(loc, height)| {
                if self.is_lowest_of_adjacents(loc.clone()) {
                    lm.insert(loc.clone(), height.saturating_add(1));
                }
            });

            self.lowest = Some(lm);
        }

        fn is_lowest_of_adjacents(&self, loc: Location) -> bool {
            let mut lowest = true;
            let height = *self.index.get(&loc).unwrap();
            // let l2 = loc.clone();
            for adj in self.adjacents_of(loc) {
                // println!("adj {:?}", adj);
                if let Some(h) = self.index.get(&adj) {
                    if *h < height {
                        lowest = false;
                        break;
                    }
                }
            }
            // println!(
            //     "checking {:?} of heigh {:?} lowest {:?}",
            //     l2, height, lowest
            // );
            lowest
        }

        fn adjacents_of(&self, loc: Location) -> Vec<Location> {
            let mut v: Vec<Location> = vec![];
            v.push(Location::new(
                loc.column.overflowing_sub(1).0,
                loc.row.overflowing_sub(1).0,
            ));
            v.push(Location::new(loc.column, loc.row.overflowing_sub(1).0));
            v.push(Location::new(
                loc.column.saturating_add(1),
                loc.row.overflowing_sub(1).0,
            ));

            v.push(Location::new(loc.column.overflowing_sub(1).0, loc.row));
            v.push(Location::new(loc.column.saturating_add(1), loc.row));

            v.push(Location::new(
                loc.column.overflowing_sub(1).0,
                loc.row.saturating_add(1),
            ));
            v.push(Location::new(loc.column, loc.row.saturating_add(1)));
            v.push(Location::new(
                loc.column.saturating_add(1),
                loc.row.saturating_add(1),
            ));
            v
        }
    }

    // Part 1: Create map & Search low points
    let mut m = Map::new(content);
    m.compute_lowest_points();
    // println!("m.lowest_set {:?}", m.lowest);
    let l: LowestMap = m.lowest.unwrap();
    println!("Part 1 : Solution is {:?}", l.values().sum::<usize>());
}
