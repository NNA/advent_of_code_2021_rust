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

    type CaveMap = HashMap<String, Vec<String>>;

    let mut map: CaveMap = CaveMap::new();

    trait Segmentable {
        fn add_segment(&mut self, start: &str, end: &str);
        fn traverse(
            &mut self,
            paths: Vec<Vec<String>>,
            allow_small_twice: bool,
        ) -> Vec<Vec<String>>;
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

        fn traverse(
            &mut self,
            paths: Vec<Vec<String>>,
            allow_small_twice: bool,
        ) -> Vec<Vec<String>> {
            let mut cloned_paths = paths.clone();
            let mut added = false;
            let mut new_paths: Vec<Vec<String>> = vec![];

            while let Some(path) = cloned_paths.pop() {
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
                            let big_cave = *neighbor == neighbor.to_uppercase();
                            let mut can_be_visited_twice = allow_small_twice;

                            if allow_small_twice {
                                let mut dedup_path: Vec<String> = path
                                    .clone()
                                    .into_iter()
                                    .filter(|x| *x != x.to_uppercase())
                                    .collect();
                                let ref_path: Vec<String> = path
                                    .clone()
                                    .into_iter()
                                    .filter(|x| *x != x.to_uppercase())
                                    .collect();
                                dedup_path.sort();
                                dedup_path.dedup();
                                can_be_visited_twice = (neighbor != "start")
                                    && (neighbor != "end")
                                    && (dedup_path.len() == ref_path.len());
                            }

                            // println!("can be visited twice {:?}", can_be_visited_twice);
                            if big_cave || !visited || can_be_visited_twice {
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
                self.traverse(new_paths, allow_small_twice)
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
    let res = map.traverse(vec![vec!["start".to_string()]], false);
    let res2 = map.traverse(vec![vec!["start".to_string()]], true);

    // println!("res {:?}", res);
    println!("Part1 : Solution is {:?}", res.len());
    println!("Part2 : Solution is {:?}", res2.len());
}
