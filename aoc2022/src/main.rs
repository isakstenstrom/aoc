use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day9;
mod util;

use crate::util::read_input_from_file;

macro_rules! run_day {
    ( $i:literal ,$run_all:ident, $day:ident, $task:ident, $use_sample:ident ) => {
        if $run_all || $day == $i {
            let task_input = if $use_sample {
                read_input_from_file(&format!("sample/day{}.txt", $i))
            } else {
                read_input_from_file(&format!("input/day{}.txt", $i))
            };
            if $task == -1 || $task == 1 {
                let task_time = Instant::now();
                println!(
                    "Day {:>2}: Task 1: {:>16}, in {:?}",
                    $i,
                    paste::expr! {[<day $i>]::task1(&task_input)},
                    task_time.elapsed()
                );
            }
            if $task == -1 || $task == 2 {
                let task_time = Instant::now();
                println!(
                    "Day {:>2}: Task 2: {:>16}, in {:?}",
                    $i,
                    paste::expr! {[<day $i>]::task2(&task_input)},
                    task_time.elapsed()
                );
            }
        }
    };
}

fn main() {
    let mut run_all = false;
    let mut day = -1;
    let mut task = -1;
    let mut use_sample_input = false;

    let mut arg_iter = std::env::args().peekable();

    // Discards the program name
    arg_iter.next();

    // Defaults to -a if no arguments are given
    if arg_iter.peek().is_none() {
        run_all = true;
    }

    while let Some(arg) = arg_iter.next() {
        match &arg[..] {
            "-a" | "--all" => run_all = true,
            "-d" | "--day" => {
                day = arg_iter
                    .next()
                    .expect("Missing task number.")
                    .parse()
                    .expect("Invalid day number. Must be an integer between 1 and 25 (including).");
                if !(1..=25).contains(&day) {
                    panic!("Invalid day number. Must be an integer between 1 and 25 (including).");
                }
            }
            "-t" | "--task" => {
                task = arg_iter
                    .next()
                    .expect("Missing task number.")
                    .parse()
                    .expect("Invalid task number. Must be either 1 or 2.");
                if !(1..=2).contains(&task) {
                    panic!("Invalid task number. Must be either 1 or 2.");
                }
            }
            "-s" | "--sample" => use_sample_input = true,
            _ => panic!("Invalid argument {}", arg),
        }
    }

    let total_time = Instant::now();

    run_day!(1, run_all, day, task, use_sample_input);
    run_day!(2, run_all, day, task, use_sample_input);
    run_day!(3, run_all, day, task, use_sample_input);
    run_day!(4, run_all, day, task, use_sample_input);
    run_day!(5, run_all, day, task, use_sample_input);
    run_day!(6, run_all, day, task, use_sample_input);
    run_day!(7, run_all, day, task, use_sample_input);
    run_day!(9, run_all, day, task, use_sample_input);
    run_day!(10, run_all, day, task, use_sample_input);
    run_day!(11, run_all, day, task, use_sample_input);
    run_day!(12, run_all, day, task, use_sample_input);
    run_day!(19, run_all, day, task, use_sample_input);

    if run_all {
        println!("Total time elapsed: {:?}", total_time.elapsed());
    }
}
