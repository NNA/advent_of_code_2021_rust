#![feature(array_windows)]

use std::{fs::File, path::Path, io::Read};

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
        Ok(_) => print!("{} contains:\n{}", display, content),
    }

    count_larger_measurements(&content);
    
    count_larger_measurements_in_windows(&content);
    
}

fn count_larger_measurements_in_windows(content: &String) {
    let mut previous_total: u16 = u16::MAX;
    let mut increase_count: u16 = 0;

    let ar: Vec<&str> = content.split('\n').collect();
    let mut total: u16;
    for window in ar.array_windows::<3>() {
        total = 0;
        for x in *window {
            match x.parse::<u16>() {
                Ok(x) => {
                    total+=x;
                },
                Err(_) => ()
            }
        }
        if total > previous_total {
            increase_count+=1;
        }
        previous_total = total;
    }

    println!("There are {} measurements that are larger than the previous measurement using windows.", increase_count);
}

fn count_larger_measurements(content: &String) {
    let mut previous_measurement: u16 = u16::MAX;
    let mut increase_count: u16 = 0;

    content.split('\n').for_each({
        |x| match x.parse::<u16>() {
            Ok(x) => {
                if x > previous_measurement {
                    increase_count+=1;
                }
                previous_measurement = x;
            },
            Err(_) => ()
        }
    });

    println!("There are {} measurements that are larger than the previous measurement.", increase_count);
}
