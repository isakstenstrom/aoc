mod day1;

fn main() {
    let day: String = std::env::args().nth(1).unwrap();

    match day.as_str() {
        "1_1" => day1::run1(),
        "1_2" => day1::run2(),
        _ => println!("Day not implemented yet."),
    };
}
