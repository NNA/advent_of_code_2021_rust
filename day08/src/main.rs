use core::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::RangeBounds;
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

    #[derive(Debug, Clone)]
    struct SignalPattern {
        active_wires: Vec<Wire>,
    }

    #[derive(Debug, Clone)]
    struct DisplayDigit {
        active_wires: Vec<Wire>,
    }

    #[derive(Debug, Clone)]
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

        fn is_missing_one_char_from(&self, other: &SignalPattern) -> bool {
            other
                .active_wires
                .iter()
                .filter(|item| !self.active_wires.contains(item))
                .count()
                == 1
        }

        fn contains_all_wires_of(&self, other: &SignalPattern) -> bool {
            let mut res = true;
            for wire in &self.active_wires {
                if !&other.active_wires.contains(wire) {
                    res = false;
                    break;
                }
            }
            res
        }
    }

    impl fmt::Display for SignalPattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.active_wires.iter().collect::<String>())
        }
    }

    impl fmt::Display for DisplayDigit {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.active_wires.iter().collect::<String>())
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

    fn length_to_digit(len: usize) -> Option<usize> {
        match len {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    type Notes = Vec<NoteEntry>;

    let mut notes: Notes = vec![];

    // Part 1
    // Parse input & create structs
    for line in content.split('\n') {
        notes.push(NoteEntry::from(line));
    }

    // let notes_part2 = notes.clone();
    // let mut easy_count = 0;
    // notes.into_iter().for_each(|note_entry| {
    //     easy_count += note_entry
    //         .output_value
    //         .iter()
    //         .filter(|digit| digit.is_easy())
    //         .count();
    // });

    // println!("Part 1: Solution {:?}", easy_count);
    let mut total = 0;

    //Part 2
    notes.into_iter().enumerate().for_each(|(i, note_entry)| {
        let mut deduced: HashMap<usize, &SignalPattern> = HashMap::new();
        let mut decoded_digits: Vec<String> = vec![];

        // Identify digit 1, 7, 4 and 8 using unique length
        note_entry.signal_patterns.iter().for_each(|signal| {
            if let Some(digit) = length_to_digit(signal.active_wires.len()) {
                deduced.insert(digit, signal);
            }
        });

        // Identify digits with length 6

        // If length is 6 and there is one char diff from signals for digit 1
        // => Then it is digit 6
        note_entry.signal_patterns.iter().for_each(|signal| {
            if signal.active_wires.len() == 6
                && deduced.get(&1).is_some()
                && signal.is_missing_one_char_from(*deduced.get(&1).unwrap())
            {
                deduced.insert(6, signal);
            }
        });

        // If length is 6 and there is one char diff from signals for digit 4
        // and it's not 6 digit
        // => Then it is digit 0
        note_entry.signal_patterns.iter().for_each(|signal| {
            if signal.active_wires.len() == 6
                && deduced.get(&4).is_some()
                && signal.is_missing_one_char_from(*deduced.get(&4).unwrap())
                && (deduced
                    .values()
                    .filter(|s| s.to_string() == signal.to_string())
                    .count()
                    == 0)
            // Because digit 6 is  1 char diff and we have already found it, let's ignore it
            {
                deduced.insert(0, signal);
            }
        });

        // If length is 6 and it's not digit 6 or digit 0
        // => Then it is digit 9
        note_entry.signal_patterns.iter().for_each(|signal| {
            if signal.active_wires.len() == 6
                && (deduced
                    .values()
                    .filter(|s| s.to_string() == signal.to_string())
                    .count()
                    == 0)
            {
                deduced.insert(9, signal);
            }
        });

        // Identify digits with length 5

        // If length is 5 and same signals has digit 6 wires
        // => Then it is digit 5
        note_entry.signal_patterns.iter().for_each(|signal| {
            if signal.active_wires.len() == 5
                && signal.contains_all_wires_of(*deduced.get(&6).unwrap())
            {
                deduced.insert(5, signal);
            }
        });

        // If length is 5 and same signals has digit 6 wires
        note_entry.signal_patterns.iter().for_each(|signal| {
            if signal.active_wires.len() == 5
                && signal.contains_all_wires_of(*deduced.get(&9).unwrap())
                // Because digit 5 is also same as 9 and we have already found it, let's ignore it
                && (deduced
                    .values()
                    .filter(|s| s.to_string() == signal.to_string())
                    .count()
                    == 0)
            {
                deduced.insert(3, signal);
            }
        });

        // If length is 5 and it's not digit 5 nor digit 3
        // => Then it is digit 2
        note_entry.signal_patterns.iter().for_each(|signal| {
            if signal.active_wires.len() == 5
                && (deduced
                    .values()
                    .filter(|s| s.to_string() == signal.to_string())
                    .count()
                    == 0)
            {
                deduced.insert(2, signal);
            }
        });

        // Reverse the deduced hash : keys are now a sorted string of Signal Patterns.
        let mut reverse: HashMap<String, String> = HashMap::new();
        deduced.into_iter().for_each(|(k, v)| {
            let mut chars: Vec<char> = v.active_wires.to_owned();
            chars.sort();
            let _res = reverse.insert(chars.iter().collect::<String>(), k.to_string());
        });

        // Seek patterns and decode the 4 digit in output
        note_entry.output_value.iter().for_each(|digit| {
            let mut sorted_digit: Vec<char> = digit.active_wires.to_owned();
            sorted_digit.sort();

            let pattern = sorted_digit.iter().collect::<String>();
            let matching_digit: String = reverse.get(&pattern).unwrap().to_string();

            decoded_digits.push(matching_digit);
        });

        // Append 4 digits into an integer add sum it
        let output_value: String = decoded_digits.into_iter().collect();
        total += output_value.parse::<usize>().unwrap();
    });

    println!("Part 2 solution is {:?}", total);
}
