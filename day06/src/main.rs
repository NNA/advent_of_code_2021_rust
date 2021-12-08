#![feature(slice_group_by)]

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

    #[derive(Debug, Clone)]
    struct Fish {
        timer: u16,
    }

    impl Fish {
        fn new(initial_timer: Option<u16>) -> Self {
            Fish {
                timer: if initial_timer.is_some() {
                    initial_timer.unwrap()
                } else {
                    8
                },
            }
        }

        fn live_for_a_day_and_say_if_a_new_fish_is_born(&mut self) -> bool {
            let mut should_create_fish: bool = false;
            if self.timer == 0 {
                self.timer = 6;
                should_create_fish = true;
            } else {
                self.timer -= 1;
            }
            should_create_fish
        }
    }

    impl From<Fish> for String {
        fn from(fish: Fish) -> Self {
            fish.timer.to_string()
        }
    }

    type FishBatch = Vec<Fish>;

    #[derive(Debug, Clone)]
    struct Sea {
        fishes: Vec<FishBatch>,
    }

    impl Sea {
        fn new(first_batch_timer_str: String) -> Self {
            let mut first_batch: FishBatch = vec![];
            for fish_timer_str in first_batch_timer_str.split(',') {
                first_batch.push(Fish::new(Some(fish_timer_str.parse().unwrap())));
            }
            Sea {
                fishes: vec![first_batch],
            }
        }

        fn spend_one_day(&mut self) {
            let mut fishes_born_in_day: FishBatch = vec![];

            for batch in &mut self.fishes {
                for fish in batch.into_iter() {
                    if fish.live_for_a_day_and_say_if_a_new_fish_is_born() {
                        fishes_born_in_day.push(Fish::new(None));
                    }
                }
            }

            self.fishes.push(fishes_born_in_day);
        }

        fn spend_n_days(&mut self, number_of_days: u16) {
            for _day_number in 1..number_of_days + 1 {
                self.spend_one_day();
            }
        }

        fn fish_count(&self) -> usize {
            let mut count = 0;
            for batch in &self.fishes {
                count += batch.len();
            }
            count
        }
    }

    type FishCountByAge = Vec<usize>;

    #[derive(Debug, Clone)]
    struct SeaPart2 {
        fishes: FishCountByAge,
    }

    impl SeaPart2 {
        fn new(first_batch_timer_str: String) -> Self {
            // let mut first_generation; //: FishCountByAge = vec![];

            let mut first_generation: Vec<u16> = first_batch_timer_str
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect();

            // We add every possible age
            first_generation.append(&mut [0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec());

            first_generation.sort();

            let fish_count_by_age: FishCountByAge = first_generation
                .group_by(|a, b| a == b)
                .map(|x| x.len() - 1) //We remove 1 because we added 1 to count also non present ages
                .collect();

            SeaPart2 {
                fishes: fish_count_by_age,
            }
        }

        fn spend_one_day(&mut self) {
            let zero_day_fish = *self.fishes.get(0).unwrap();
            // We reduce age of every fish by 1
            self.fishes.rotate_left(1);
            // Breeding fishes have now the age of 6
            *self.fishes.get_mut(6).unwrap() += zero_day_fish;
            // Born fishes have now the age to 8
            *self.fishes.get_mut(8).unwrap() = zero_day_fish;
        }

        fn spend_n_days(&mut self, number_of_days: u16) {
            for _day_number in 1..number_of_days + 1 {
                self.spend_one_day();
            }
        }

        fn fish_count(&self) -> usize {
            self.fishes.iter().sum()
        }
    }

    println!("Initial state: {}", content);

    // Part 1 : Simple algorithm
    let content2 = content.clone();
    let mut sea1: Sea = Sea::new(content);
    sea1.spend_n_days(80);

    println!("Part 1: Solution is {:?}", sea1.fish_count());

    // Part2 : Simple algorithm doesn't work, optimize by applying rotate.
    let mut sea2 = SeaPart2::new(content2);
    sea2.spend_n_days(256);

    println!("Part 2: Solution is {:?}", sea2.fish_count());
}
