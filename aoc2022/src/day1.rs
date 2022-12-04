use crate::util::read_input_as_string;

// Answer should be "69281"
pub fn task1() {
    let input = read_input_as_string("day1.txt");
    println!(
        "{}",
        input
            .split("\n\n")
            .map(|l| {
                l.split('\n')
                    .map(|c| c.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
    );
}

// Answer should be "201524"
pub fn task2() {
    let input = read_input_as_string("day1.txt");
    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;
    input
        .split("\n\n")
        .map(|l| {
            l.split('\n')
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .for_each(|c| {
            if c > max1 {
                max1 = c;
            } else if c > max2 {
                max2 = c;
            } else if c > max3 {
                max3 = c;
            }
        });
    println!("{}", max1 + max2 + max3);
}
