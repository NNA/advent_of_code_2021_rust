use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn main() {
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

    let mut frequency_array = [[0 as u16; 2]; 12];

    // Part 1
    for bin_number in content.split('\n') {
        for (pos, bit) in bin_number.chars().enumerate() {
            match bit {
                '0' => frequency_array[pos][0] += 1,
                '1' => frequency_array[pos][1] += 1,
                _ => panic!("invalid binary number"),
            }
        }
    }

    let binary_rates: (String, String) = extract_binary_rates(frequency_array);

    let gamma = u32::from_str_radix(binary_rates.0.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(binary_rates.1.as_str(), 2).unwrap();

    println!(
        "PART 1 - At the end : gamma & epsilon are [{:?}, {:?}] so number is [{}]",
        gamma,
        epsilon,
        gamma * epsilon
    );

    // Part 2
    let split_content_oxygen: Vec<&str> = content.split('\n').collect();

    let mut oxygen_rating = String::new();

    for i in 0..=11 {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut candidates_count = 0;
        let mut last_candidate: &str = "";
        for bin_number in &split_content_oxygen {
            if !bin_number.starts_with(&oxygen_rating) {
                continue;
            }
            candidates_count += 1;
            last_candidate = bin_number;
            match bin_number.chars().nth(i) {
                Some(c) => match c {
                    '0' => zero_count += 1,
                    '1' => one_count += 1,
                    _ => panic!("invalid binary number"),
                },
                None => (),
            }
        }
        if candidates_count == 1 {
            oxygen_rating = last_candidate.to_string();
            break;
        }
        if zero_count > one_count {
            oxygen_rating.push('0')
        } else {
            oxygen_rating.push('1')
        }
    }

    let mut co2_rating = String::new();

    for i in 0..=11 {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut candidates_count = 0;
        let mut last_candidate: &str = "";
        for bin_number in &split_content_oxygen {
            if !bin_number.starts_with(&co2_rating) {
                continue;
            }
            candidates_count += 1;
            last_candidate = bin_number;
            match bin_number.chars().nth(i) {
                Some(c) => match c {
                    '0' => zero_count += 1,
                    '1' => one_count += 1,
                    _ => panic!("invalid binary number"),
                },
                None => (),
            }
        }
        if candidates_count == 1 {
            co2_rating = last_candidate.to_string();
            break;
        }
        if zero_count > one_count {
            co2_rating.push('1')
        } else {
            co2_rating.push('0')
        }
    }

    let oxygen = u32::from_str_radix(oxygen_rating.as_str(), 2).unwrap();
    let co2 = u32::from_str_radix(co2_rating.as_str(), 2).unwrap();

    println!(
        "PART 2 - At the end : oxygen & co2 are [{:?}, {:?}] so number is [{}]",
        oxygen,
        co2,
        oxygen * co2
    );
}

fn extract_binary_rates(frequency_array: [[u16; 2]; 12]) -> (String, String) {
    let mut gamma = String::with_capacity(12);
    let mut epsilon = String::with_capacity(12);

    for frequency in frequency_array.iter() {
        if frequency[0] > frequency[1] {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    (gamma, epsilon)
}
