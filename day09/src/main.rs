use std::collections::{HashMap, HashSet};
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
    type Bassin = HashSet<Location>;

    type MapIndex = HashMap<Location, Height>;
    type LowestMap = HashMap<Location, RiskLevel>;
    type BassinVec = Vec<(Bassin, usize)>;

    #[derive(Debug)]
    struct Map {
        row_count: usize,
        col_count: usize,
        index: MapIndex,
        lowest: Option<LowestMap>,
        bassins: Option<BassinVec>,
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
                bassins: None,
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
            lowest
        }

        fn adjacents_of(&self, loc: Location) -> Vec<Location> {
            let mut v: Vec<Location> = vec![];

            if loc.column > 0 && loc.row > 0 {
                v.push(Location::new(loc.column - 1, loc.row - 1));
            }
            if loc.row > 0 {
                v.push(Location::new(loc.column, loc.row - 1));
            }
            if loc.column < self.col_count && loc.row > 0 {
                v.push(Location::new(loc.column + 1, loc.row - 1));
            }

            if loc.column > 0 {
                v.push(Location::new(loc.column - 1, loc.row));
            }
            if loc.column < self.col_count {
                v.push(Location::new(loc.column + 1, loc.row));
            }

            if loc.column > 0 && loc.row < self.row_count {
                v.push(Location::new(loc.column - 1, loc.row + 1));
            }
            if loc.row < self.row_count {
                v.push(Location::new(loc.column, loc.row + 1));
            }
            if loc.column < self.col_count && loc.row < self.row_count {
                v.push(Location::new(loc.column + 1, loc.row + 1));
            }
            v
        }

        fn floodable_adjacents_of(&self, loc: Location) -> Vec<Location> {
            let mut v: Vec<Location> = vec![];

            // Checking x-1;y-1
            if loc.column > 0 && loc.row > 0 {
                // Check its 2 neighbors (x, y-1) & (x-1, y) allow pass (ie < 9)
                let n1 = self
                    .index
                    .get(&Location::new(loc.column, loc.row - 1))
                    .unwrap();
                let n2 = self
                    .index
                    .get(&Location::new(loc.column - 1, loc.row))
                    .unwrap();
                let n = self
                    .index
                    .get(&Location::new(loc.column - 1, loc.row - 1))
                    .unwrap();
                if (n1 < &9 || n2 < &9) && n < &9 {
                    v.push(Location::new(loc.column - 1, loc.row - 1));
                }
            }
            // Checking x;y-1
            if loc.row > 0 {
                let n = self
                    .index
                    .get(&Location::new(loc.column, loc.row - 1))
                    .unwrap();
                if n < &9 {
                    v.push(Location::new(loc.column, loc.row - 1));
                }
            }
            // Checking x+1;y-1
            if loc.column < self.col_count - 1 && loc.row > 0 {
                // Check its 2 neighbors (x, y-1) & (x+1, y) allow pass (ie < 9)
                let n1 = self
                    .index
                    .get(&Location::new(loc.column, loc.row - 1))
                    .unwrap();
                let n2 = self
                    .index
                    .get(&Location::new(loc.column + 1, loc.row))
                    .unwrap();
                let n = self
                    .index
                    .get(&Location::new(loc.column + 1, loc.row - 1))
                    .unwrap();
                if (n1 < &9 || n2 < &9) && n < &9 {
                    v.push(Location::new(loc.column + 1, loc.row - 1));
                }
            }

            // Checking x-1;y
            if loc.column > 0 {
                let n = self
                    .index
                    .get(&Location::new(loc.column - 1, loc.row))
                    .unwrap();
                if n < &9 {
                    v.push(Location::new(loc.column - 1, loc.row));
                }
            }

            // Checking x+1;y
            if loc.column < self.col_count - 1 {
                let n = self
                    .index
                    .get(&Location::new(loc.column + 1, loc.row))
                    .unwrap();
                if n < &9 {
                    v.push(Location::new(loc.column + 1, loc.row));
                }
            }

            // Checking x-1;y+1
            if loc.column > 0 && loc.row < self.row_count - 1 {
                // Check its 2 neighbors (x-1, y) & (x, y+1) allow pass (ie < 9)
                let n1 = self
                    .index
                    .get(&Location::new(loc.column - 1, loc.row))
                    .unwrap();
                let n2 = self
                    .index
                    .get(&Location::new(loc.column, loc.row + 1))
                    .unwrap();
                let n = self
                    .index
                    .get(&Location::new(loc.column - 1, loc.row + 1))
                    .unwrap();
                if (n1 < &9 || n2 < &9) && n < &9 {
                    v.push(Location::new(loc.column - 1, loc.row + 1));
                }
            }

            // Checking x;y+1
            if loc.row < self.row_count - 1 {
                let n = self
                    .index
                    .get(&Location::new(loc.column, loc.row + 1))
                    .unwrap();
                if n < &9 {
                    v.push(Location::new(loc.column, loc.row + 1));
                }
            }

            if loc.column < self.col_count - 1 && loc.row < self.row_count - 1 {
                // Check its 2 neighbors (x, y+1) & (x+1, y) allow pass (ie < 9)
                let n1 = self
                    .index
                    .get(&Location::new(loc.column, loc.row + 1))
                    .unwrap();
                let n2 = self
                    .index
                    .get(&Location::new(loc.column + 1, loc.row))
                    .unwrap();
                let n = self
                    .index
                    .get(&Location::new(loc.column + 1, loc.row + 1))
                    .unwrap();
                if (n1 < &9 || n2 < &9) && n < &9 {
                    v.push(Location::new(loc.column + 1, loc.row + 1));
                }
            }

            v
        }

        fn compute_bassins(&mut self) {
            let mut bl: BassinVec = vec![];

            self.lowest
                .clone()
                .unwrap()
                .into_iter()
                .for_each(|(loc, _risk)| {
                    let mut candidates_bassin_members: Vec<Location> =
                        self.floodable_adjacents_of(loc.clone());
                    let mut bassin_members = HashSet::new();
                    let mut checked_candidates: HashSet<Location> = HashSet::new();
                    checked_candidates.insert(loc);

                    while let Some(c) = candidates_bassin_members.pop() {
                        checked_candidates.insert(c.clone());
                        for adj in self.floodable_adjacents_of(c) {
                            if !bassin_members.contains(&adj) {
                                bassin_members.insert(adj.clone());
                            }
                            if !checked_candidates.contains(&adj) {
                                candidates_bassin_members.push(adj.clone());
                            }
                            checked_candidates.insert(adj.clone());
                        }
                    }
                    let members_count = bassin_members.iter().count();
                    // println!(
                    //     "new Bassin found {:?} members => {:?}",
                    //     members_count, bassin_members
                    // );
                    bl.push((bassin_members, members_count));
                });

            self.bassins = Some(bl);
        }
    }

    // Part 1: Create map & Search low points
    let mut m = Map::new(content);
    m.compute_lowest_points();
    let l: LowestMap = m.lowest.clone().unwrap();
    println!("Part 1 : Solution is {:?}", l.values().sum::<usize>());

    // Part 2: Compute bassins
    m.compute_bassins();
    let b: BassinVec = m.bassins.unwrap();
    let mut b_sizes: Vec<usize> = b.into_iter().map(|(_k, v)| v).collect();
    b_sizes.sort();
    b_sizes.reverse();
    println!(
        "Part 2 : Solution is {:?}",
        b_sizes.into_iter().take(3).reduce(|acc, x| acc * x)
    );
}
