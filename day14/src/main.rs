use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::slice::SliceIndex;

type PolymerTemplate = String;
type Rules = HashMap<(char, char), char>;

#[derive(Debug, PartialEq, Clone)]
struct Manual {
    template: PolymerTemplate,
    rules: Rules,
}

impl Manual {
    fn new(content: String) -> Self {
        let mut rules = Rules::new();

        let mut split = content.split("\n\n");
        let template = split.next().unwrap().to_string();
        let rules_str = split.next().unwrap();

        for rule in rules_str.lines() {
            let mut iter = rule.split(" -> ");
            let src: String = iter.next().unwrap().to_string().parse().unwrap();
            let dest = iter.next().unwrap().to_string().parse().unwrap();
            rules.insert(
                (src.chars().nth(0).unwrap(), src.chars().nth(1).unwrap()),
                dest,
            );
        }

        Manual { template, rules }
    }
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("input_test.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut content = String::new();
    if let Err(why) = file.read_to_string(&mut content) {
        panic!("couldn't read {}: {}", display, why)
    }

    // Part 1
    let mut manual: Manual = Manual::new(content);
    println!("Part1 solution is {:?}", manual);
}
