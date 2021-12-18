use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input_test_small.txt");
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
    struct CavePap {
        index: HashMap<String, Vec<String>>,
    }

    type CaveMap = HashMap<String, Vec<String>>;

    let mut map: CaveMap = CaveMap::new();

    trait Segmentable {
        fn add_segment(&mut self, start: String, end: String);
    }

    impl Segmentable for CaveMap {
        fn add_segment(&mut self, start: String, end: String) {
            match self.get_mut(&start) {
                Some(existing) => {
                    existing.push(end.to_string());
                }
                None => {
                    self.insert(start.to_string(), vec![end.to_string()]);
                }
            }
        }
    }

    for segment in content.lines() {
        let mut iter = segment.split('-');
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();

        map.add_segment(start.to_string(), end.to_string());
        if end.to_string() == end.to_ascii_uppercase() {
            map.add_segment(end.to_string(), start.to_string());
        }
    }

    println!("map {:?}", map);
}
