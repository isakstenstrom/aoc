mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

fn main() {
    let day: String = std::env::args().nth(1).unwrap();

    match day.as_str() {
        "1_1" => day1::task1(),
        "1_2" => day1::task2(),
        "2_1" => day2::task1(),
        "2_2" => day2::task2(),
        "3_1" => day3::task1(),
        "3_2" => day3::task2(),
        "4_1" => day4::task1(),
        "4_2" => day4::task2(),
        "5_1" => day5::task1(),
        "5_2" => day5::task2(),
        _ => println!("Day not implemented yet."),
    };
}
