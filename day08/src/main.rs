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

    type Wire = char;

    #[derive(Debug)]
    struct SignalPattern {
        active_wires: Vec<Wire>,
    }

    #[derive(Debug)]
    struct DisplayDigit {
        active_wires: Vec<Wire>,
    }

    #[derive(Debug)]
    struct NoteEntry {
        signal_patterns: [SignalPattern; 10],
        output_value: [DisplayDigit; 4],
    }

    impl SignalPattern {
        fn new(wires: String) -> Self {
            Self {
                active_wires: wires.chars().collect(),
            }
        }
    }

    impl DisplayDigit {
        fn new(wires: String) -> Self {
            Self {
                active_wires: wires.chars().collect(),
            }
        }

        fn is_easy(&self) -> bool {
            match self.active_wires.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            }
        }
    }

    impl From<&str> for NoteEntry {
        fn from(entry: &str) -> Self {
            let mut iter = entry.split('|');
            let signals_iter = iter.next().unwrap();
            let outputs_iter = iter.next().unwrap();

            let mut signals: Vec<SignalPattern> = vec![];
            let mut outputs: Vec<DisplayDigit> = vec![];

            signals_iter
                .trim_end()
                .split(' ')
                .for_each(|x| signals.push(SignalPattern::new(x.to_string())));

            outputs_iter
                .trim_start()
                .split(' ')
                .for_each(|x| outputs.push(DisplayDigit::new(x.to_string())));

            NoteEntry {
                signal_patterns: signals.try_into().unwrap_or_else(|v: Vec<_>| {
                    panic!("Expected a Vec of length 10 but it was {}", v.len())
                }),
                output_value: outputs.try_into().unwrap_or_else(|v: Vec<_>| {
                    panic!("Expected a Vec of length 4 but it was {}", v.len())
                }),
            }
        }
    }

    type Notes = Vec<NoteEntry>;

    let mut notes: Notes = vec![];

    // Part 1
    // Parse input & create structs
    for line in content.split('\n') {
        notes.push(NoteEntry::from(line));
    }

    // println!("Notes {:?}", notes);

    let mut easy_count = 0;
    notes.into_iter().for_each(|note_entry| {
        easy_count += note_entry
            .output_value
            .iter()
            .filter(|digit| digit.is_easy())
            .count();
    });

    println!("Part 1: Solution {:?}", easy_count);
}
