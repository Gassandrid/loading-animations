use std::io;
use std::io::Write;
use std::time::Duration;
use num_traits::Float;
use rand::seq::SliceRandom;
use std::{thread, time};

pub fn loading_bar1(message: &str, length: u32, speed: f64) {
    let mut i = 0;
    let mut percent: f64 = 0.0;
    let percent_increment: f64 = 100.0 / length as f64;
    
    let mut final_word = message.chars().collect::<Vec<_>>();
    let mut scrambled: Vec<char> = final_word.clone();
    scrambled.shuffle(&mut rand::thread_rng());

    let label_speed = &speed/4.0;

    for i in 0..final_word.len() {
        while final_word[i] != scrambled[i] {
            let temp = scrambled[i];
            scrambled.remove(i);
            scrambled.push(temp);
            print!("\r{}", scrambled.iter().collect::<String>());
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs_f64(label_speed));
        }
    }
    print!("\n");

    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    while i <= length {
        print!("\r[");
        for _ in 0..i {
            print!("█");
        }
        for _ in 0..length - i {
            print!("░");
        }
        let rounded_percent = format!("{:.2}", percent); // Round the percent value to 2 decimal places using format!
        print!("] {}% ", rounded_percent); // Print the rounded percent value with 2 decimal places
        percent += percent_increment;
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs_f64(speed));
        i += 1;
    }
}



fn shuffle_sort_anim(string: &str, speed: f64) {
    
}