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
}

fn extract_binary_rates(frequency_array: [[u16; 2]; 12]) -> (String, String) {
    let mut gamma = ['0'; 12];
    let mut epsilon = ['0'; 12];

    for (i, frequency) in frequency_array.iter().enumerate() {
        if frequency[0] > frequency[1] {
            gamma[i] = '1';
            epsilon[i] = '0';
        } else {
            gamma[i] = '0';
            epsilon[i] = '1';
        }
    }

    (gamma.into_iter().collect(), epsilon.into_iter().collect())
}
