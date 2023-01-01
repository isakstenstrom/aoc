use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_input_from_file(filename: &str) -> Vec<String> {
    let file = File::open("res/".to_owned() + filename).expect("Error: input file does not exist");
    io::BufReader::new(file).lines().flatten().collect()
}
