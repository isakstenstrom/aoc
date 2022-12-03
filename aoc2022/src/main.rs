mod day1;
mod day2;
mod day3;
mod util;

fn main() {
    let day: String = std::env::args().nth(1).unwrap();

    match day.as_str() {
        "1_1" => day1::run1(),
        "1_2" => day1::run2(),
        "2_1" => day2::run1(),
        "2_2" => day2::run2(),
        "3_1" => day3::run1(),
        "3_2" => day3::run2(),
        _ => println!("Day not implemented yet."),
    };
}
