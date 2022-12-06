mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn main() {
    let day: String = std::env::args().nth(1).unwrap();

    match day.as_str() {
        "1_1" => println!("{}", day1::task1()),
        "1_2" => println!("{}", day1::task2()),
        "2_1" => println!("{}", day2::task1()),
        "2_2" => println!("{}", day2::task2()),
        "3_1" => println!("{}", day3::task1()),
        "3_2" => println!("{}", day3::task2()),
        "4_1" => println!("{}", day4::task1()),
        "4_2" => println!("{}", day4::task2()),
        "5_1" => println!("{}", day5::task1()),
        "5_2" => println!("{}", day5::task2()),
        "6_1" => println!("{}", day6::task1()),
        "6_2" => println!("{}", day6::task2()),
        _ => println!("Day not implemented yet."),
    };
}
