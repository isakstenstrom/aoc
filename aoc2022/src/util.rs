use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines_from_input(filename: &str) -> Vec<String> {
    let file =
        File::open("res/input/".to_owned() + filename).expect("Error: input file does not exist");
    io::BufReader::new(file).lines().flatten().collect()
}
