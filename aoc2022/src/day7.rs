use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::util::read_input_from_file;

fn create_file_table(lines: &Vec<String>) -> HashMap<String, u32> {
    let mut file_table: HashMap<String, u32> = HashMap::new();
    let mut path = PathBuf::new();
    path.push("/");

    for line in lines {
        if line.starts_with("$ cd") {
            if line[5..] == ".."[..] {
                path.pop();
            } else {
                path.push(line[5..].to_string());
            }
        } else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            let mut split_line = line.split(' ');
            let file_size = str::parse::<u32>(split_line.next().unwrap()).unwrap();
            let file_name = split_line.next().unwrap();

            file_table
                .entry(path.to_str().unwrap().to_string() + "/" + file_name)
                .or_insert(file_size);
        }
    }
    file_table
}

fn create_dir_table(file_table: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut dir_table: HashMap<String, u32> = HashMap::new();
    for (file_path, size) in file_table {
        let p = Path::new(&file_path);
        let mut path_iter = p.ancestors();
        path_iter.next();

        while path_iter.peekable().peek().is_some() {
            let dir_name = path_iter.next().unwrap().to_str().unwrap().to_string();
            let dir_size = dir_table.entry(dir_name).or_insert(0);
            *dir_size += size;
        }
    }
    dir_table
}

pub fn task1() -> u32 {
    let lines = read_input_from_file("day7.txt");

    create_dir_table(&create_file_table(&lines))
        .iter()
        .filter(|d| d.1 <= &100000u32)
        .map(|d| d.1)
        .sum::<u32>()
}

pub fn task2() -> u32 {
    let lines = read_input_from_file("day7.txt");

    let dir_table = create_dir_table(&create_file_table(&lines));

    let space_needed = 30000000 - (70000000 - dir_table.get("/").unwrap());

    *dir_table
        .iter()
        .filter(|d| d.1 >= &space_needed)
        .map(|d| d.1)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day7::{task1, task2};

    #[test]
    fn test_task1() {
        assert_eq!(task1(), 1644735);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 1300850);
    }
}
