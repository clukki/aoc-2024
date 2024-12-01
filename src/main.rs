use std::{env, fs};

mod day1;

// PLEASE MODIFY THIS FILE IF YOU WANT TO RUN ANY ENTRY
fn main() {
    let input_file_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input_file_path).unwrap();

    println!("{}", day1::part2(input))
}
