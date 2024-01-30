use std::io;
use std::io::BufRead;
use std::fs::File;
use rand::Rng; // Add this line to import the `rand` crate.

pub struct ProceduralGenerator {
    filename: String,
    lines: Vec<String>,
}

impl ProceduralGenerator {
    pub fn new(filename: String) -> ProceduralGenerator {
        let file = File::open(&filename).unwrap();
        let lines: Vec<String> = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect();
        ProceduralGenerator { filename, lines }
    }

    pub fn return_random_line(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.lines.len());
        self.lines[random_index].to_string()
    }

    pub fn return_random_speed(&self) -> f64 {
        let mut rng = rand::thread_rng();
        let random_speed = rng.gen_range(0.001..0.1);
        random_speed
    }

    pub fn return_random_length(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let random_length = rng.gen_range(10..115);
        random_length
    }
}
