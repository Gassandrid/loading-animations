use std::env;
use std::thread;
use std::time::Duration;
use loading_bar::*; // Import the loading_bar module
use procedural_generator::*; // Import the procedural_generator module

mod loading_bar;
mod procedural_generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = args[1].parse().unwrap();

    let pg = ProceduralGenerator::new("src/modules.txt".to_string());

    for i in 0..input {
        let mut random_line = pg.return_random_line();
        let mut random_speed = pg.return_random_speed();
        let mut random_length = pg.return_random_length();
        loading_bar1(&random_line, random_length, random_speed);
        thread::sleep(Duration::from_secs_f32(pg.return_random_speed() as f32));
        println!(); // Add a new line after every iteration
    }
}
