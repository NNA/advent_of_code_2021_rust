use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::{self, Path};
use std::{thread, time};

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

    // #[derive(Debug)]
    // struct CavePap {
    //     index: HashMap<str, Vec<str>>,
    // }

    type CaveMap = HashMap<String, Vec<String>>;

    let mut map: CaveMap = CaveMap::new();

    trait Segmentable {
        fn add_segment(&mut self, start: &str, end: &str);
        fn traverse(&mut self, paths: Vec<Vec<String>>) -> Vec<Vec<String>>;
    }

    impl Segmentable for CaveMap {
        fn add_segment(&mut self, start: &str, end: &str) {
            self.entry(start.to_string())
                .or_default()
                .push(end.to_string());
            self.entry(end.to_string())
                .or_default()
                .push(start.to_string());
        }

        fn traverse(&mut self, paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
            // println!("******** NEW traverse");
            // println!("paths {:?}", paths);
            let mut cloned_paths = paths.clone();
            let mut added = false;
            let mut new_paths: Vec<Vec<String>> = vec![];

            while let Some(path) = cloned_paths.pop() {
                // println!("poping path {:?}", path);
                let last_element = path.last().unwrap();
                if *last_element == "end".to_string() {
                    new_paths.push(path);
                    continue;
                }
                match self.get_mut(last_element) {
                    Some(neighborlist) => {
                        let base_path = path.clone();
                        neighborlist.iter().for_each(|neighbor| {
                            let visited = path.contains(neighbor);
                            let allowed_multiple = *neighbor == neighbor.to_uppercase();
                            if (allowed_multiple || !visited) {
                                let mut cloned = base_path.clone();
                                cloned.push(neighbor.clone());
                                // println!("new ongoing path {:?}", cloned);
                                new_paths.push(cloned);
                                added = true;
                            }
                        });
                    }
                    None => (),
                }
            }

            // println!("new_paths {:?}", new_paths);
            if added {
                // At least one path has increased, we need to try to increase it.
                self.traverse(new_paths)
            } else {
                // No paths added, we cannot continue => we only ouput those that end at "end"
                paths
                    .into_iter()
                    .filter(|r| *r.last().unwrap() == "end".to_string())
                    .collect()
            }
        }
    }

    // Build the map by adding segments
    for segment in content.lines() {
        let mut iter = segment.split('-');
        map.add_segment(iter.next().unwrap(), iter.next().unwrap());
    }

    // Traverse recursively
    let res = map.traverse(vec![vec!["start".to_string()]]);

    // println!("res {:?}", res);
    println!("Part1 : Solution is {:?}", res.len());
}
