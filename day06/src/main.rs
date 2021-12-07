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

    fn spend_one_day(fishes: &mut Vec<Fish>) {
        let mut fishes_born_in_day: Vec<Fish> = vec![];
        for mut fish in fishes.into_iter() {
            if fish.live_for_a_day_and_say_if_a_new_fish_is_born() {
                fishes_born_in_day.push(Fish::new(None));
            }
        }
        fishes.append(&mut fishes_born_in_day);
    }

    let mut fishes: Vec<Fish> = vec![];

    println!("Initial state: {}", content);

    // Part 1 : 372300

    // Parse input & prepare LanternFishes
    for fish_timer_str in content.split(',') {
        fishes.push(Fish::new(Some(fish_timer_str.parse().unwrap())));
    }

    let mut number_of_days = 80;
    println!("Part 1: Solution is {:?}", fishes.len());

    for day_number in 1..number_of_days + 1 {
        spend_one_day(&mut fishes);
        // println!(
        //     "After {: >2} days: {}",
        //     day_number.to_string(),
        //     fishes
        //         .clone()
        //         .into_iter()
        //         .map(|f| String::from(f) + ",")
        //         .collect::<String>()
        // );
    }

    // Part2
    number_of_days = 256;
    fishes = vec![];

    for fish_timer_str in content.split(',') {
        fishes.push(Fish::new(Some(fish_timer_str.parse().unwrap())));
    }

    for day_number in 1..number_of_days + 1 {
        println!("Day {:?}", day_number);
        spend_one_day(&mut fishes);
    }
    println!("Part 2: Solution is {:?}", fishes.len());
}
