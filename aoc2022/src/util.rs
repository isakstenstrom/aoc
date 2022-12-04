use std::{
    fs::File,
    io::{self, BufRead, Read},
};

pub fn read_input_as_lines(filename: &str) -> Vec<String> {
    let file =
        File::open("res/input/".to_owned() + filename).expect("Error: input file does not exist");
    io::BufReader::new(file).lines().flatten().collect()
}

pub fn read_input_as_string(filename: &str) -> String {
    let file =
        File::open("res/input/".to_owned() + filename).expect("Error: input file does not exist");
    let mut res: String = String::new();
    io::BufReader::new(file).read_to_string(&mut res).unwrap();
    res
}
