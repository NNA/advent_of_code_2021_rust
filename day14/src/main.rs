use std::collections::{HashMap, HashSet};
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

    fn step(&mut self) {
        let mut insertions = HashMap::new();
        for i in 0..self.template.len() - 1 {
            let first = self.template[i..i + 1].chars().nth(0).unwrap();
            let second = self.template[i + 1..i + 2].chars().nth(0).unwrap_or('*');
            if let Some(dest) = self.rules.get(&(first, second)) {
                // println!("match found {} {} => {:?}", first, second, dest);
                // We keep the place and element that should be inserted
                insertions.insert(i + 1, dest);
            }
        }

        // insertions has positions & elemnts that should be inserted
        // println!("insertions {:?}", insertions);
        let capa = self.template.len() + insertions.len();
        let mut new_template = String::with_capacity(capa);
        self.template.chars().enumerate().for_each(|(i, c)| {
            if insertions.contains_key(&i) {
                new_template.push(**insertions.get(&i).unwrap());
            }
            new_template.push(c);
        });
        self.template = new_template;
    }

    fn frequencies(&self) -> HashMap<char, u32> {
        let mut freq = HashMap::new();
        for ch in self.template.chars() {
            let counter = freq.entry(ch).or_insert(0);
            *counter += 1;
        }
        freq
    }
}

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
    if let Err(why) = file.read_to_string(&mut content) {
        panic!("couldn't read {}: {}", display, why)
    }

    // Part 1
    let mut manual: Manual = Manual::new(content);
    // println!("Part1 solution is {:?}", manual);
    for i in 1..=10 {
        manual.step();
        println!("after step {:?}", i);
        // println!("after step {} template is {:?} ", i, manual.template);
    }

    let freq = manual.frequencies();
    let max = freq.values().max().unwrap();
    let min = freq.values().min().unwrap();
    println!("max is {:?}, min is {:?}", max, min);

    println!("Part1 result is {:?}", *max - *min);
}
